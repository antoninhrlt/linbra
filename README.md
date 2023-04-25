# linbra
Easily do linear algebra in game development, graphics and other sorts of calculations using vectors and matrices.

Every implementation, function or item is documented mathematically and programmatically. Browses the [documentation](https://docs.rs/linbra/latest/linbra) in order to find the items and their functions to learn how to use them!

## Overview
- Objects:

| Mathematics | Linbra | Related types |
| --- | --- | --- |
| $\begin{pmatrix} x_{1,1} & x_{1,2} & \dots & x_{1,C} \\ x_{2,1} & x_{2,2} & \dots & x_{2,C} \\ \vdots & \vdots & \ddots & \vdots \\ x_{R,1} & x_{R,2} & \dots & x_{R,C} \\ \end{pmatrix}$ | [`Matrix<T, C, R>`](https://docs.rs/linbra/latest/linbra/matrix/struct.Matrix.html) | <ul><li>[`Matrix2<T>`](https://docs.rs/linbra/latest/linbra/matrix/type.Matrix2.html)</li> <li>[`Matrix3<T>`](https://docs.rs/linbra/latest/linbra/matrix/type.Matrix3.html)</li> <li>[`Matrix4<T>`](https://docs.rs/linbra/latest/linbra/matrix/type.Matrix4.html)</li></ul> |
| $\begin{pmatrix} a_{1} \\ a_{2} \\ \vdots \\ a_{n} \\ \end{pmatrix}$ | [`Vector<T, N>`](https://docs.rs/linbra/latest/linbra/vector/struct.Vector.html) | <ul><li>[`Vector2<T>`](https://docs.rs/linbra/latest/linbra/vector/type.Vector2.html)</li> <li>[`Vector3<T>`](https://docs.rs/linbra/latest/linbra/vector/type.Vector3.html)</li> <li>[`Vector4<T>`](https://docs.rs/linbra/latest/linbra/vector/type.Vector3.html)</li></ul> |

- Tools:

| Mathematics | Related traits | and their functions |
| --- | --- | --- |
| $\begin{pmatrix} x \\ y \\ \end{pmatrix} or \begin{pmatrix} x \\ y \\ z \\ \end{pmatrix}$ | <ul><li>[`Point2<T>`](https://docs.rs/linbra/latest/linbra/points/trait.Point2.html)</li><li>[`Point3<T>`](https://docs.rs/linbra/latest/linbra/points/trait.Point3.html)</li></ul> | <ul><li>`::x()`</li><li>`::y()`</li><li>`::z()`</li></ul> |
| $\begin{pmatrix} w \\ h \\ \end{pmatrix} or \begin{pmatrix} w \\ h \\ d \\ \end{pmatrix}$ | <ul><li>[`Size2<T>`](https://docs.rs/linbra/latest/linbra/sizes/trait.Size2.html)</li><li>[`Size3<T>`](https://docs.rs/linbra/latest/linbra/sizes/trait.Size3.html)</li></ul> | <ul><li>`::w()`</li><li>`::h()`</li><li>`::d()`</li></ul> |
| $\begin{pmatrix} r \\ g \\ b \\ \end{pmatrix} or \begin{pmatrix} r \\ g \\ b \\ a \\ \end{pmatrix}$ | <ul><li>[`RGB<T>`](https://docs.rs/linbra/latest/linbra/colours/trait.RGB.html)</li><li>[`RGBA<T>`](https://docs.rs/linbra/latest/linbra/colours/trait.RGBA.html)</li></ul> | <ul><li>`::r()`</li><li>`::g()`</li><li>`::b()`</li><li>`::a()`</li></ul> |


## Note
This project is under development.
