#! /usr/bin/env nix-shell
#! nix-shell -i python ../shell.nix

import os
import http.server
import socketserver

PORT = 4000

os.chdir(os.path.join(os.path.dirname(os.path.abspath(__file__)), "../", "_site"))

TCPServer = socketserver.TCPServer
TCPServer.allow_reuse_address = True

HttpRequestHandler = http.server.SimpleHTTPRequestHandler
HttpRequestHandler.extensions_map.update({'.wasm': 'application/wasm'})

with TCPServer(("", PORT), HttpRequestHandler) as httpd:
    print("serving at port", PORT)
    httpd.serve_forever()
