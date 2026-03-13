# AiOpenComputer 計畫

> 設計一台電腦，包含軟體和硬體

> 重建整個軟體工業文明

> 本計劃由 AI 輔助設計

本計劃選定使用 rust 程式語言，實作下列子計畫軟體

[AiCompiler]:https://github.com/aiopencomputer/aicompiler
[AiOs]:https://github.com/aiopencomputer/aios
[AiCpu]:https://github.com/aiopencomputer/aicpu
[AiBrowser]:https://github.com/aiopencomputer/aibrowser
[AiEda]:https://github.com/aiopencomputer/aieda
[AiAi]:https://github.com/aiopencomputer/aiai

1. [AiCompiler] -- 編譯器，中間碼用 LLVM IR，最後轉為 iCPU 可以執行的機器碼。
2. [AiOs] -- 作業系統，必須可以在 iCPU 處理器上執行。
3. [AiCpu] -- 處理器，相容於 iMac M3 晶片
4. [AiBrowser] -- 瀏覽器，支援 HTML / CSS / JavaScript 標準
5. [AiEda] -- 電子設計工具，支援 iHDL，是融合 rust 與 Verilog 的硬體的描述語言。
6. [AiAi] -- 人工智慧，包含『神經網路引擎+語言模型+Agent』(類似 PyTorch+GPT+OpenClaw)



