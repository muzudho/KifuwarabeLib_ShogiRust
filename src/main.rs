use std::io;

fn main() {
    loop{
        // 入力行
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .ok()// read_lineの返り値オブジェクトResult の okメソッド
            .expect("info Failed to read line");// OKで無かった場合のエラーメッセージ

        let line : String = line.trim().parse()
            .ok()
            .expect("info Failed to parse");

        let len = line.chars().count();

        // goで始まるコマンドが来たら投了
        if 1<len && &line[0..2] == "go" { println!("bestmove resign"); break; }
        // positionで始まるコマンドが来たら無視
        else if 7<len && &line[0..8] == "position" {}
        else {
            match line.as_ref() {
                "usi" =>{
                    println!("id name Kifuwarabe_Rust");
                    println!("id author Muzudho");
                    println!("usiok")
                },
                "isready" => println!("readyok"),
                _ =>{},
            }
        }
    }
}
