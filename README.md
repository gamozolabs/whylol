# Summary

In common bleeding edge securty resarch applications like Symbolic Execution,
people are inhibited by their abilities to process and parse complex
architectures like x86. With this solution, we use modern web development
processes and machine learning to solve those problems and get expressions
from common machine code. Since machine learning is used here, it can be
scaled out to deploy these security solutions to many problems in the industry.
Providing radical change.

# Solutions used

- Machine learning
- Rust <3
- Modern HTML
- High-performance string parsing
- Complex image compression
- JSON and well-structured serialization and deserialization

# PRs

I am currently accepting PRs! Let me know if you want to contribute and improve
this project.

```
pleb@gamey ~/whylol $ cargo run --release
   Compiling whylol v0.1.0 (/home/pleb/whylol)
    Finished release [optimized] target(s) in 0.82s
     Running `target/release/whylol`
/media/FLcfnWwXMAA6gMn.png
Decoding as middle english
Warning: Invalid resolution 0 dpi. Using 70 instead.
Got expr: CMP quord ptr (R12 + RAX*0x8] , 0x0
Solved pointer: R12 + RAX*0x8
```

