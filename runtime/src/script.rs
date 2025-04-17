use vlitz_shared::{VlitzError, VlitzResult};

/// 기본 RPC 스크립트 템플릿
const BASE_SCRIPT_TEMPLATE: &str = r#"
(() => {
    "use strict";
    
    // RPC 호출 핸들러 등록
    rpc.exports = {
        // 메모리 읽기
        readMemory: function(address, size, type) {
            let ptr = new NativePointer(address);
            
            switch (type) {
                case "byte": return ptr.readS8();
                case "ubyte": return ptr.readU8();
                case "short": return ptr.readS16();
                case "ushort": return ptr.readU16();
                case "int": return ptr.readS32();
                case "uint": return ptr.readU32();
                case "long": return ptr.readS64();
                case "ulong": return ptr.readU64();
                case "float": return ptr.readFloat();
                case "double": return ptr.readDouble();
                case "bool": return ptr.readU8() !== 0;
                case "pointer": return ptr.readPointer().toString();
                case "string": return ptr.readUtf8String();
                case "bytes": return ptr.readByteArray(size);
                default: throw new Error(`Unsupported memory type: ${type}`);
            }
        },
        
        // 메모리 쓰기
        writeMemory: function(address, value, type) {
            let ptr = new NativePointer(address);
            
            switch (type) {
                case "byte": ptr.writeS8(value); break;
                case "ubyte": ptr.writeU8(value); break;
                case "short": ptr.writeS16(value); break;
                case "ushort": ptr.writeU16(value); break;
                case "int": ptr.writeS32(value); break;
                case "uint": ptr.writeU32(value); break;
                case "long": ptr.writeS64(value); break;
                case "ulong": ptr.writeU64(value); break;
                case "float": ptr.writeFloat(value); break;
                case "double": ptr.writeDouble(value); break;
                case "bool": ptr.writeU8(value ? 1 : 0); break;
                case "pointer": ptr.writePointer(ptr(value)); break;
                case "string": ptr.writeUtf8String(value); break;
                default: throw new Error(`Unsupported memory type: ${type}`);
            }
            return true;
        },
        
        // 메모리 범위 스캔
        scanMemory: function(pattern, rangeAddress, rangeSize, protection) {
            let matches = [];
            let ranges = Process.enumerateRangesSync(protection || "---");
            
            if (rangeAddress) {
                ranges = ranges.filter(range => {
                    const start = parseInt(range.base);
                    const end = start + range.size;
                    const targetStart = parseInt(rangeAddress);
                    const targetEnd = targetStart + (rangeSize || 0);
                    
                    return (start <= targetStart && targetStart < end) || 
                           (start <= targetEnd && targetEnd < end) ||
                           (targetStart <= start && end <= targetEnd);
                });
            }
            
            for (const range of ranges) {
                const hits = Memory.scanSync(range.base, range.size, pattern);
                matches.push(...hits);
            }
            
            return matches;
        },
        
        // 모듈 열거
        enumerateModules: function() {
            return Process.enumerateModules();
        },
        
        // 익스포트 열거
        enumerateExports: function(moduleName) {
            return Process.getModuleByName(moduleName).enumerateExports();
        },
        
        // 범위 열거
        enumerateRanges: function(protection) {
            return Process.enumerateRanges(protection || "---");
        },
        
        // 클래스 열거 (Java/ObjC)
        enumerateClasses: function() {
            if (Java.available) {
                return Java.enumerateLoadedClassesSync();
            } else if (ObjC.available) {
                return ObjC.classes;
            }
            return [];
        },
        
        // 메서드 열거 (Java)
        enumerateJavaMethods: function(className) {
            if (!Java.available) return [];
            
            const methods = [];
            Java.perform(() => {
                const clazz = Java.use(className);
                for (const method of Object.getOwnPropertyNames(clazz.__proto__)) {
                    if (method === 'constructor') continue;
                    
                    const overloads = clazz[method].overloads;
                    for (const overload of overloads) {
                        const args = overload.argumentTypes.map(t => t.className);
                        methods.push({
                            name: method,
                            returnType: overload.returnType.className,
                            argumentTypes: args
                        });
                    }
                }
            });
            
            return methods;
        },
        
        // 함수 후킹
        hookFunction: function(address) {
            const hookId = "hook_" + address;
            
            Interceptor.attach(ptr(address), {
                onEnter: function(args) {
                    send({
                        type: "functionHook",
                        id: hookId,
                        event: "enter",
                        address: address,
                        threadId: this.threadId,
                        args: Array.from({length: 8}, (_, i) => args[i].toString())
                    });
                },
                onLeave: function(retval) {
                    send({
                        type: "functionHook",
                        id: hookId,
                        event: "leave", 
                        address: address,
                        threadId: this.threadId,
                        retval: retval.toString()
                    });
                }
            });
            
            return hookId;
        },
        
        // 함수 호출
        callFunction: function(address, args, returnType) {
            const func = new NativeFunction(ptr(address), returnType || 'void', 
                Array(args.length).fill('pointer'));
            
            const nativeArgs = args.map(arg => ptr(arg));
            return func.apply(null, nativeArgs);
        },
        
        // Java 메서드 후킹
        hookJavaMethod: function(className, methodName, argumentTypes) {
            if (!Java.available) {
                throw new Error("Java API is not available");
            }
            
            const hookId = `java_hook_${className}_${methodName}`;
            
            Java.perform(() => {
                const clazz = Java.use(className);
                let method = clazz[methodName];
                
                if (argumentTypes && argumentTypes.length > 0) {
                    method = method.overload(...argumentTypes);
                }
                
                method.implementation = function(...args) {
                    const argValues = [];
                    for (let i = 0; i < args.length; i++) {
                        try {
                            argValues.push(args[i].toString());
                        } catch (e) {
                            argValues.push(`<non-serializable: ${e.message}>`);
                        }
                    }
                    
                    send({
                        type: "javaMethodHook",
                        id: hookId,
                        event: "enter",
                        className: className,
                        methodName: methodName,
                        args: argValues
                    });
                    
                    const retval = this[methodName](...args);
                    
                    send({
                        type: "javaMethodHook",
                        id: hookId,
                        event: "leave",
                        className: className,
                        methodName: methodName,
                        retval: retval ? retval.toString() : "null"
                    });
                    
                    return retval;
                };
            });
            
            return hookId;
        },
        
        // Java 메서드 호출
        callJavaMethod: function(className, methodName, argumentTypes, thisObject, args) {
            if (!Java.available) {
                throw new Error("Java API is not available");
            }
            
            let result;
            Java.perform(() => {
                const clazz = Java.use(className);
                let method = clazz[methodName];
                
                if (argumentTypes && argumentTypes.length > 0) {
                    method = method.overload(...argumentTypes);
                }
                
                if (thisObject) {
                    const instance = Java.cast(thisObject, clazz);
                    result = method.apply(instance, args || []);
                } else {
                    result = method.apply(null, args || []);
                }
            });
            
            return result ? result.toString() : null;
        }
    };
})();
"#;

/// 스크립트 유틸리티
pub struct ScriptUtils;

impl ScriptUtils {
    /// 기본 RPC 스크립트 가져오기
    pub fn get_base_script() -> &'static str {
        BASE_SCRIPT_TEMPLATE
    }

    /// 스크립트 파일에서 로드
    pub fn load_from_file(path: &str) -> VlitzResult<String> {
        std::fs::read_to_string(path)
            .map_err(|e| VlitzError::Io(e))
    }

    /// 메모리 후킹 스크립트 생성
    pub fn create_memory_watch_script(address: u64, memory_type: &str) -> String {
        format!(
            r#"
(() => {{
    "use strict";
    
    const address = "0x{:x}";
    const memoryType = "{}";
    let lastValue = null;
    
    const memoryAccessMonitor = {{
        address: ptr(address),
        onAccess: function(details) {{
            send({{
                type: "memoryAccess",
                address: address,
                operation: details.operation, // read or write
                from: details.from.toString(),
                threadId: details.threadId
            }});
        }}
    }};
    
    // 초기값 읽기
    const getValue = () => {{
        const ptr = new NativePointer(address);
        
        switch (memoryType) {{
            case "byte": return ptr.readS8();
            case "ubyte": return ptr.readU8();
            case "short": return ptr.readS16();
            case "ushort": return ptr.readU16();
            case "int": return ptr.readS32();
            case "uint": return ptr.readU32();
            case "long": return ptr.readS64();
            case "ulong": return ptr.readU64();
            case "float": return ptr.readFloat();
            case "double": return ptr.readDouble();
            case "bool": return ptr.readU8() !== 0;
            case "pointer": return ptr.readPointer().toString();
            case "string": return ptr.readUtf8String();
            default: throw new Error(`Unsupported memory type: ${{memoryType}}`);
        }}
    }};
    
    // 초기값 저장
    lastValue = getValue();
    
    // 값 변경 감시
    const watchInterval = setInterval(() => {{
        const currentValue = getValue();
        
        if (JSON.stringify(currentValue) !== JSON.stringify(lastValue)) {{
            send({{
                type: "memoryChanged",
                address: address,
                memoryType: memoryType,
                oldValue: lastValue,
                newValue: currentValue
            }});
            
            lastValue = currentValue;
        }}
    }}, 100);
    
    // 메모리 접근 감시
    MemoryAccessMonitor.enable([memoryAccessMonitor]);
    
    // RPC 익스포트
    rpc.exports = {{
        // 감시 중지
        stopWatch: function() {{
            clearInterval(watchInterval);
            MemoryAccessMonitor.disable([memoryAccessMonitor]);
            return true;
        }},
        
        // 현재 값 가져오기
        getValue: function() {{
            return getValue();
        }}
    }};
}})();
            "#,
            address,
            memory_type
        )
    }

    /// 메모리 잠금 스크립트 생성
    pub fn create_memory_lock_script(address: u64, memory_type: &str, value: &str) -> String {
        format!(
            r#"
(() => {{
    "use strict";
    
    const address = "0x{:x}";
    const memoryType = "{}";
    const lockValue = {};
    
    const writeValue = () => {{
        const ptr = new NativePointer(address);
        
        switch (memoryType) {{
            case "byte": ptr.writeS8(lockValue); break;
            case "ubyte": ptr.writeU8(lockValue); break;
            case "short": ptr.writeS16(lockValue); break;
            case "ushort": ptr.writeU16(lockValue); break;
            case "int": ptr.writeS32(lockValue); break;
            case "uint": ptr.writeU32(lockValue); break;
            case "long": ptr.writeS64(lockValue); break;
            case "ulong": ptr.writeU64(lockValue); break;
            case "float": ptr.writeFloat(lockValue); break;
            case "double": ptr.writeDouble(lockValue); break;
            case "bool": ptr.writeU8(lockValue ? 1 : 0); break;
            case "pointer": ptr.writePointer(ptr(lockValue)); break;
            case "string": ptr.writeUtf8String(lockValue); break;
            default: throw new Error(`Unsupported memory type: ${{memoryType}}`);
        }}
    }};
    
    // 초기 값 설정
    writeValue();
    
    // 값 락 유지
    const lockInterval = setInterval(() => {{
        writeValue();
    }}, 50);
    
    // RPC 익스포트
    rpc.exports = {{
        // 잠금 중지
        stopLock: function() {{
            clearInterval(lockInterval);
            return true;
        }},
        
        // 현재 잠금 값 가져오기
        getLockValue: function() {{
            return lockValue;
        }}
    }};
}})();
            "#,
            address,
            memory_type,
            value
        )
    }

    /// 함수 추적 스크립트 생성
    pub fn create_function_trace_script(address: u64) -> String {
        format!(
            r#"
(() => {{
    "use strict";
    
    const targetAddress = "0x{:x}";
    let callCount = 0;
    let callHistory = [];
    
    Interceptor.attach(ptr(targetAddress), {{
        onEnter: function(args) {{
            const timestamp = new Date().getTime();
            const callInfo = {{
                id: callCount++,
                timestamp: timestamp,
                threadId: this.threadId,
                backtrace: Thread.backtrace(this.context, Backtracer.ACCURATE)
                    .map(DebugSymbol.fromAddress),
                args: Array.from({{length: 8}}, (_, i) => args[i].toString())
            }};
            
            callHistory.push(callInfo);
            
            // 최대 100개 기록 유지
            if (callHistory.length > 100) {{
                callHistory.shift();
            }}
            
            send({{
                type: "functionTrace",
                event: "call",
                address: targetAddress,
                callInfo: callInfo
            }});
            
            // 콜 스택 저장
            this.callInfo = callInfo;
        }},
        
        onLeave: function(retval) {{
            if (this.callInfo) {{
                this.callInfo.returnValue = retval.toString();
                this.callInfo.duration = new Date().getTime() - this.callInfo.timestamp;
                
                send({{
                    type: "functionTrace",
                    event: "return",
                    address: targetAddress,
                    returnValue: retval.toString(),
                    callInfo: this.callInfo
                }});
            }}
        }}
    }});
    
    // RPC 익스포트
    rpc.exports = {{
        // 호출 횟수 조회
        getCallCount: function() {{
            return callCount;
        }},
        
        // 호출 기록 조회
        getCallHistory: function() {{
            return callHistory;
        }},
        
        // 호출 기록 초기화
        clearCallHistory: function() {{
            callHistory = [];
            return true;
        }}
    }};
}})();
            "#,
            address
        )
    }
} 