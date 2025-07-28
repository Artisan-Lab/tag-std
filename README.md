# tag-std

This project aims to provide an annotation-based approach to managing safety-related comments in unsafe Rust code. It has three main objectives:
- The annotation system should be user-friendly and compatible with existing developer workflows.
- The annotations should be syntactically checkable by the compiler, thereby enabling standardized usage.
- If sufficiently precise, the annotations could also support formal verification, for example by being extended into contracts. However, this is not required for general projects.

The project is named **tag-std** because it originally aims to standardize safety property annotations for the Rust core and standard libraries. We have already defined a set of [primitive safety properties](primitive-sp.md) to describe the safety concerns associated with unsafe APIs in the standard library, and we have [labeled these unsafe APIs](data/std.json) accordingly. In addition, we have developed a systematic method to detect and resolve discrepancies through program analysis. For more details, please refer to our paper:
- "[Annotating and Auditing the Safety Properties of Unsafe Rust](https://arxiv.org/abs/2504.21312)", Zihao Rao, Hongliang Tian, Xin Wang, **Hui Xu**, _arXiv:2504.21312_, 2025.

While we are formulating the [annotation method](usage.md) and developing the [corresponding tools](safety-tool), we are also exploring the application of this approach to Rust projects beyond the standard library, including [Rust-for-Linux](https://github.com/rust-for-linux) and [Asterinas](https://github.com/asterinas/asterinas)
