fn main() {
    /*
     ** Range
     */

    // `1..3` はファーストクラス？ → Yesみたい Range<> 型の値
    let a = [1, 2, 3];
    let b = 1..3;

    println!("{a:?}");
    println!("{b:?}");
    // println!("{}", b[1]); // これはできない

    /*
     ** 配列
     */

    // mut がないと変更できない JavaScriptのconstとは違う
    let mut arr = [1, 2];

    arr[0] = 10;
    let a1 = arr[0];
    let a2 = arr[1];
    // let a3 = arr[2]; // コンパイルエラーになる

    let i = 2; // usizeに推測される 頭いいな

    // let a3 = arr[i]; // これもコンパイルエラーになる 頭いい

    println!("{:?} {:?}", a1, a2);

    /*
     ** ベクタ
     */

    // mut がないと変更できない JavaScriptのconstとは違う
    let mut v = vec![1, 2];

    let a = v[0];
    let b = v[1];
    // let c = v[2]; // コンパイルエラーにならず、実行するとpanicする しかしOptionではない
    println!("{:?} {:?}", a, b);

    let x = v.pop();
    let y = v.pop();
    let z = v.pop();
    println!("{:?} {:?} {:?}", x, y, z);

    /*
     ** 割り算
     */
    // let q = 1 / 0; // 警告されてそれでも実行するとpanic Optionじゃない → あとでやったらエラーになった
    // println!("{}", q);
}
