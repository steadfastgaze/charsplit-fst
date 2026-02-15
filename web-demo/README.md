# German Compound Word Splitter - Web Demo

This is a web-based demonstration of the German compound word splitter using WebAssembly.

## How to Deploy

1. Build the WASM module:
   ```bash
   ./build-wasm.sh
   ```

2. Copy the FST data files:
   ```bash
   mkdir -p web-demo/data
   cp data/*.fst web-demo/data/
   ```

3. Copy the WASM files:
   ```bash
   cp -r pkg-web web-demo/
   ```

4. Serve the `web-demo` directory using a web server.

## Alternative: Using Brotli Compression

To reduce file sizes, you can compress the FST files with Brotli:

```bash
./compress-data.sh
```

Then update the HTML file to load the `.br` files instead of the raw `.fst` files.

## Local Testing

You can test locally using Python's built-in server:

```bash
cd web-demo
python -m http.server 8000
```

Then open http://localhost:8000 in your browser.

Note: The demo runs entirely in the browser using WebAssembly. No server-side processing is required.

## GitHub Pages Deployment

To deploy to GitHub Pages:

1. Build the WASM module: `./build-wasm.sh`
2. Copy data files: `cp data/*.fst web-demo/data/`
3. Copy WASM files: `cp -r pkg-web web-demo/`
4. Push the `web-demo` directory to your GitHub Pages branch

The demo will be available at `https://<username>.github.io/<repository>/web-demo/`
