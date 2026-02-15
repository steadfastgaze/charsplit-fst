# WASM Build of charsplit-fst

## Building

To build the WASM version:

```bash
./build-wasm.sh
```

This will create a `pkg` directory with the compiled WASM module and JavaScript bindings.

## Files

- `charsplit_fst.js` - JavaScript bindings for the WASM module
- `charsplit_fst_bg.wasm` - The compiled WASM binary
- `suffix.fst.br`, `prefix.fst.br`, `infix.fst.br` - Brotli-compressed FST data files

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
    <script src="pkg-web/charsplit_fst.js"></script>
    <script>
        async function initApp() {
            // Load the WASM module
            const wasm = await import('./pkg-web/charsplit_fst.js');
            
            // Helper function to decompress Brotli data
            async function decompressBrotli(response) {
                const decompressedStream = response.body
                    .pipeThrough(new DecompressionStream('br'));
                const decompressedResponse = new Response(decompressedStream);
                return await decompressedResponse.arrayBuffer();
            }

            // Load the Brotli-compressed FST data files
            const [suffixData, prefixData, infixData] = await Promise.all([
                fetch('data/suffix.fst.br').then(decompressBrotli),
                fetch('data/prefix.fst.br').then(decompressBrotli),
                fetch('data/infix.fst.br').then(decompressBrotli)
            ]);
            
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

The build script automatically sets up the complete `web-demo/` directory:

```bash
./build-wasm.sh
```

This script:
- Builds the WASM module to `pkg-web/`
- Copies WASM package to `web-demo/pkg-web/`
- Creates Brotli-compressed FST files in `web-demo/data/`

Then copy the entire `web-demo/` directory to your web server.

**Browser support:** The DecompressionStream API is used for client-side Brotli decompression. Supported in all modern browsers.
