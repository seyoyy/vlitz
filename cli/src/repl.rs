use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use rustyline::config::Configurer;
use vlitz_core::{Command, CommandExecutor, CommandResult};
use anyhow::{Result as AnyhowResult, anyhow};
use colored::*;

pub struct Repl {
    editor: DefaultEditor,
    executor: CommandExecutor,
}

impl Repl {
    pub fn new(items_per_page: usize) -> AnyhowResult<Self> {
        let mut editor = DefaultEditor::new()?;
        
        // 히스토리 파일 설정
        editor.set_auto_add_history(true);
        
        // 명령어 자동완성 설정 (추후 확장 가능)
        
        let executor = CommandExecutor::new(items_per_page);
        
        Ok(Self {
            editor,
            executor,
        })
    }
    
    pub fn run(&mut self) -> AnyhowResult<()> {
        println!("{}", "VLITZ - Frida CLI Debugger".bold().green());
        println!("Type {} for help", "help".cyan());
        
        loop {
            let prompt = self.executor.get_prompt();
            
            // 라인 읽기
            let readline = self.editor.readline(&prompt);
            match readline {
                Ok(line) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    
                    // 종료 명령어 처리
                    if trimmed == "exit" || trimmed == "quit" {
                        println!("Exiting...");
                        break;
                    }
                    
                    // 명령어 파싱
                    match Command::parse(trimmed) {
                        Ok(command) => {
                            // 명령어 실행
                            match self.executor.execute(&command) {
                                CommandResult::Success(msg) => {
                                    if !msg.is_empty() {
                                        println!("{}", msg);
                                    }
                                },
                                CommandResult::Error(err) => {
                                    println!("{}: {}", "Error".red(), err);
                                },
                                CommandResult::DataList(data_list) => {
                                    for (idx, data) in data_list {
                                        let display = format!("[{}] [{}] {}", 
                                            idx, 
                                            data.data_type.to_string().cyan(), 
                                            data.get_display_name()
                                        );
                                        println!("{}", display);
                                    }
                                },
                                CommandResult::Exit => {
                                    println!("Exiting...");
                                    break;
                                },
                            }
                        },
                        Err(e) => {
                            println!("{}: {}", "Parse error".red(), e);
                        }
                    }
                },
                Err(ReadlineError::Interrupted) => {
                    println!("Ctrl-C");
                    break;
                },
                Err(ReadlineError::Eof) => {
                    println!("Ctrl-D");
                    break;
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        
        Ok(())
    }
} 