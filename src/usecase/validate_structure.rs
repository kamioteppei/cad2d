use ndarray::prelude::*;

pub fn run() {
    // 行列の定義
    let mat1 = arr2(&[
        [0.7, -0.7],
        [0.7, 0.7]
    ]);
    // println!("{}", mat1);
    // println!("{:?}", mat1.shape());

    // // 1行目・2列目の値にアクセス
    // println!("{}", mat1[[1, 2]]);

    let mat2 = arr2(&[
        [200.0],
        [200.0]
    ]);

    // 行列のアダマール積
    // let m = &mat1 * &mat2;
    // println!("{}", m);

    // 転置
    // let m = &mat1.t();
    // println!("{}", m);

    // 行列の内積
    let m = mat1.dot(&mat2);
    println!("{}", m);

    // // 行列とベクトルの積 (v = Ax, このときxは列ベクトル扱い)
    // let x = arr1(&[1, 2, 3]);
    // let v = mat1.dot(&x);
    // println!("{}", v);

    // // ベクトルと行列の積 (v = xA, このときxは行ベクトル扱い)
    // let v = &x.dot(&mat1);
    // println!("{}", v);
}