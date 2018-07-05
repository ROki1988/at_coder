fn main() {
    // stdinのロックを取得し、StdinReaderを作る。
    let stdin = std::io::stdin();
    let mut r = utils::StdinReader::new(stdin.lock());

    let count = r.readl::<usize>();  // 次の改行まで読む

    let mut xs: Vec<u32> = Vec::new();
    for i in 0..count - 1 {
        xs.push(r.reads::<u32>());
    }
    xs.push(r.readl::<u32>());
    xs.sort();

    println!("{:}",xs.last().unwrap() -  xs.first().unwrap());
}

mod utils {

    use std::io::BufRead;
    use std::str::{self, FromStr};

    pub struct StdinReader<R: BufRead> {
        reader: R,
        buf: Vec<u8>,
    }

    impl<R: BufRead> StdinReader<R> {
        pub fn new(reader: R) -> StdinReader<R> {
            StdinReader {
                reader: reader,
                buf: Vec::new(),
            }
        }

        // 区切り文字：スペース
        pub fn reads<T: FromStr>(&mut self) -> T {
            self.read_until(b' ')
        }

        // 区切り文字：改行
        pub fn readl<T: FromStr>(&mut self) -> T {
            self.read_until(b'\n')
        }

        pub fn read_until<T: FromStr>(&mut self, delim: u8) -> T {
            // self.bufに次のトークンをセットする
            loop {
                self.buf.clear();
                let len = self.reader
                    .read_until(delim, &mut self.buf)
                    .expect("failed reading bytes");
                match len {
                    0 => panic!("early eof"),
                    1 if self.buf[0] == delim => (), // 区切り文字だけなのでもう一度ループ
                    _ => {
                        // トークンが得られた
                        // 最後の文字が区切り文字なら削除
                        if self.buf[len - 1] == delim {
                            self.buf.truncate(len - 1);
                        }
                        break; // ループから脱出
                    }
                }
            }

            // 文字列をT型へパースする
            let elem = str::from_utf8(&self.buf).expect("invalid utf-8 string");
            elem.parse().unwrap_or_else(|_| panic!(format!("failed parsing: {}", elem)))
        }
    }

}