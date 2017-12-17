extern crate lazy_static;

use std::io;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::cell::RefCell;

// グローバル変数の作り方
thread_local!(
    // ログ（書き込み・追加モード）
    pub static LOG: RefCell<BufWriter<File>> ={
        let m = BufWriter::new(
            OpenOptions::new().write(true).append(true).open("log.txt").unwrap());
        RefCell::new(m)
    }
);

// 標準出力、ログ出力。末尾に改行を付加
fn g_writeln( s : &str){
    println!("{}", s);
    g_logln(s);
}
// ログ出力。末尾に改行を付加
fn g_logln( s : &str){
    let bytes = s.as_bytes();
    // グローバル変数を利用
    LOG.with(|log| {
        log.borrow_mut().write(bytes).unwrap();
        log.borrow_mut().write("\n".as_bytes()).unwrap();
    })
}

fn main() {
    loop{
        // 入力行
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .ok()// read_lineの返り値オブジェクトResult の okメソッド
            .expect("info Failed to read line");// OKで無かった場合のエラーメッセージ

        // 改行を除く
        let line : String = line.trim().parse().ok().expect("info Failed to parse");
        g_logln(&line);

        let len = line.chars().count();

        // goで始まるコマンドが来たら投了
        if 1<len && &line[0..2] == "go" {
            g_writeln("bestmove resign");
            break;
        }
        // positionで始まるコマンドが来たら無視
        else if 7<len && &line[0..8] == "position" {}
        else {
            match line.as_ref() {
                "usi" =>{
                    g_writeln("id name Kifuwarabe_Rust");
                    g_writeln("id author Muzudho");
                    g_writeln("usiok")
                },
                "isready" => g_writeln("readyok"),
                _ =>{},
            }
        }
    }
}
