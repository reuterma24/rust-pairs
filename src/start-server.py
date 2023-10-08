#Use to create local host
import http.server
import socketserver

PORT = 5173

Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map.update({
    ".js": "application/javascript",
});

print(f"Serving on port {PORT}")
httpd = socketserver.TCPServer(("", PORT), Handler)
httpd.serve_forever()