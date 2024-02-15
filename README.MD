## Training on Issues Data

This project is a precursor to the [Issue-Labeler project](https://github.com/jaykchen/issue-labeler). The rationale is to train/fine-tune a smaller Open Source model on the issues data from the WasmEdge project, so that the model can do a job that previously required more capapble and costly commercial AI inference services.

This project
- Crawled the issues data from https://github.com/WasmEdge/WasmEdge. 
- Reads the issue's title, body, creator, and labels.
- Extracting key factual information from the issue's body with ChatGPT.
- Builds a trainning dataset for the Issue-Labeler project. One entry of the training dataset is like following:
  
```json
{"completion":"The labels for this issue are `enhancement`.","prompt":"Can you assign labels to the GitHub issue titled `feat: Enabling LLM fine tuning in the WASI-NN ggml plugin` created by `hydai`, stating `WasmEdge aims to enable LLM application portability across CPUs and GPUs through the use of the WASI-NN API. The project seeks to extend the WASI-NN spec to support fine-tuning features and implement them in the WasmEdge GGML plugin by calling functions in llama.cpp. Key outcomes include using llama2-7b as the base LLM, extending the WASI-NN spec if necessary, and implementing the fine-tuning functions. Documentation, examples, tutorials, and demonstrations are also required. Key skills needed include C++, WebAssembly, and LLM fine-tuning. Notably, a GPU device is not necessary for this task.`?"}
```

- The training data is further formatted into what is needed for trainning on Google Colab: https://colab.research.google.com/drive/1tPXhQlgRkj_DpuaRQp_sU1T24MHIaxyY


- The fine-tuned model is saved to https://huggingface.co/jaykchen/tiny/tree/main.


For practical reasons, the model is hosted on a 4vGPU/8G CPU only cloud instance with Huggingface Text Generation Inference toolset at http://43.129.206.18:3000/generate

