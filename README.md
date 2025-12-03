# keyword-tools

<img src="../LOGO.png" alt="LOGO" width=150, height=150 />

## Get Started

### for Rust

#### Installation

```bash
cargo add keyword-tools
```

#### Usage

```rust
use keyword_tools::{extract_keywords, load_keywords, load_keywords_from_rsc, Language};
let keywords = load_keywords();
// keywords = load_keywords_from_rsc(PATH_TO_JSON_FILE);
let text = "After the introduction of Large Language Models (LLMs), there have been substantial improvements in the performance of Natural Language Generation (NLG) tasks, including Text Summarization and Machine Translation.";

let extracted_kwds = extract_keywords(text, Language::English);
```

## Use your keywords

If you want to use your own keywords, just create your a JSON file according to the following instructions:

```json
[
  {
    "category": "Topic",
    "language": "English",
    "word": "Edge AI",
    "alias": "Edge AI",
    "score": 10
  },
]
```

Then, load the keywords.

```rust
let kws = load_keywords_from_rsc("PATH_TO_YOUR JSON_FILE")
```

The JSON file defines a collection of topic entries used by the crate. Each entry is an object with the following fields:

### category

Specifies the category of the entry.  
Available options:

- Topic
- Security
- ComputerVision
- Organization
- MachineLearning
- Item
- NaturalLanguageProcessing

### Language

Indicates the language of the entry.  
Available options:

- Japanese
- English

### word

The main term or keyword for the topic.

### alias

An alternative name or synonym for the word.

### score

A numerical value representing the importance or relevance of the topic.
Available options:
Any integer or floating-point number (e.g., 1, 10). Higher values may indicate greater importance.
