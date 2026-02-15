# WASM Build for German Compound Word Splitter

This directory contains the WASM build of the German compound word splitter.

## Building

To build the WASM version:

```bash
./build-wasm.sh
```

This will create a `pkg-web` directory with the compiled WASM module and JavaScript bindings.

## Files

- `german_splitter.js` - JavaScript bindings for the WASM module
- `german_splitter_bg.wasm` - The compiled WASM binary
- `suffix.fst.br`, `prefix.fst.br`, `infix.fst.br` - Compressed FST data files (Brotli compressed)

## Usage

The WASM module can be used in a web page as follows:

```html
<!DOCTYPE html>
<html>
<head>
    <title>German Compound Word Splitter Demo</title>
</head>
<body>
    <div id="app">
        <h1>German Compound Word Splitter</h1>
        <input type="text" id="word-input" placeholder="Enter a German compound word..." />
        <button onclick="splitWord()">Split Word</button>
        <div id="results"></div>
    </div>

    <!-- Load the WASM module -->
    <script src="pkg-web/german_splitter.js"></script>
    <script>
        async function initApp() {
            // Load the WASM module
            const wasm = await import('./pkg-web/german_splitter.js');
            
            // Load the FST data files
            const suffixData = await fetch('data/suffix.fst.br').then(r => r.arrayBuffer());
            const prefixData = await fetch('data/prefix.fst.br').then(r => r.arrayBuffer());
            const infixData = await fetch('data/infix.fst.br').then(r => r.arrayBuffer());
            
            // Create the splitter
            const splitter = new wasm.WebSplitter(new Uint8Array(suffixData), new Uint8Array(prefixData), new Uint8Array(infixData));
            
            window.splitter = splitter; // Make it globally available for the demo
        }
        
        async function splitWord() {
            if (!window.splitter) {
                alert('Splitter not initialized yet!');
                return;
            }
            
            const word = document.getElementById('word-input').value;
            if (!word) {
                alert('Please enter a word to split');
                return;
            }
            
            const results = window.splitter.split_compound(word);
            displayResults(results);
        }
        
        function displayResults(results) {
            const resultsDiv = document.getElementById('results');
            resultsDiv.innerHTML = '<h3>Results:</h3>';
            
            if (results.length === 0) {
                resultsDiv.innerHTML += '<p>No splits found.</p>';
                return;
            }
            
            const ul = document.createElement('ul');
            for (let i = 0; i < Math.min(results.length, 10); i++) { // Show top 10
                const li = document.createElement('li');
                li.textContent = `${results[i].score.toFixed(3)} | ${results[i].part1} | ${results[i].part2}`;
                ul.appendChild(li);
            }
            resultsDiv.appendChild(ul);
        }
        
        // Initialize the app when the page loads
        initApp();
    </script>
</body>
</html>
```

## Deployment

For deployment to GitHub Pages, you'll need to:

1. Build the WASM module using `./build-wasm.sh`
2. Compress the FST data files with Brotli: `brotli data/*.fst`
3. Copy the `pkg-web` directory and compressed data files to your web server
4. Create an HTML page that loads the WASM module and data files