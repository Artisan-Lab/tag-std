# tag-std

This project aims to standardize the safety property annotation of the Rust core and standard library. There are three steps:
- Define the [primitive safety properties](primitive-sp.md) to be used for describing the safety concerns of unsafe APIs.
- [Label the unsafe APIs](usage.md) in Rust's core and standard library with primitive safety properties.
- Detect and solve discripencies via program analysis.

Through this project, we aim to establish a foundation for Rust unsafe code annotation, contract design and verification, serving as a preliminary step toward this goal.

**For more details, please refer to our paper:**
- "[Annotating and Auditing the Safety Properties of Unsafe Rust](https://arxiv.org/abs/2504.21312)", Zihao Rao, Hongliang Tian, Xin Wang, **Hui Xu**, _arXiv:2504.21312_, 2025. (corresponding author)
