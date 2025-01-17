build with cargo build --release --target x86_64-unknown-linux-musl
you'll find the release under target/x86_64-unknown-linux-musl/release/node_mem_log


Default Values:
```
-p = 5 processes per log
-l = 10 log-entries kept in rotational log
-s = 10 seconds between logs 
```

Example Output
```
=== Node.js Processes Check: 2025-01-17 18:49:37 ===

Frontend Process:
PID: 292625 | Memory: 105.63 MB | CPU: 0.0% | /nix/store/frchj7ynjnj5rwbhhdkwig25lz41y854-nodejs-20.11.1/bin/node /nix/store/sh0mn6qin1zakmcgyacrm26wbjs4fskx-corepack-nodejs-20.11.1/bin/yarn start

Top Memory Node Processes:
PID: 5674 | Memory: 696.97 MB | CPU: 2.2% | /vscode/vscode-server/bin/linux-x64/fabdb6a30b49f79a7aba0f2ad9df9b399473380f/node --dns-result-order=ipv4first /vscode/vscode-server/bin/linux-x64/fabdb6a30b49f79a7aba0f2ad9df9b399473380f/out/bootstrap-fork --type=extensionHost --transformURIs --useHostProxy=true
PID: 42164 | Memory: 627.64 MB | CPU: 0.9% | /vscode/vscode-server/bin/linux-x64/fabdb6a30b49f79a7aba0f2ad9df9b399473380f/node --max-old-space-size=3072 /workspaces/horus/frontend/.yarn/sdks/typescript/lib/tsserver.js --useInferredProjectPerProjectRoot --enableTelemetry --cancellationPipeName /tmp/vscode-typescript1000/144cb6769dc94d06d1a5/tscancellation-a69fd01593a540a192b9.tmp* --globalPlugins typescript-deno-plugin,@vscode/copilot-typescript-server-plugin --pluginProbeLocations /home/seoulrobotics/.vscode-server/extensions/denoland.vscode-deno-3.43.2,/home/seoulrobotics/.vscode-server/extensions/github.copilot-chat-0.23.2 --locale en --noGetErrOnBackgroundUpdate --validateDefaultNpmLocation --useNodeIpc
PID: 184994 | Memory: 341.87 MB | CPU: 0.9% | /usr/share/code/code --type=utility --utility-sub-type=node.mojom.NodeService --lang=en-US --service-sandbox-type=none --dns-result-order=ipv4first --inspect-port=0 --crashpad-handler-pid=3924 --enable-crash-reporter=ecf269fb-51a8-4b4a-9839-b932a80428b9,no_channel --user-data-dir=/home/julian/.config/Code --standard-schemes=vscode-webview,vscode-file --enable-sandbox --secure-schemes=vscode-webview,vscode-file --cors-schemes=vscode-webview,vscode-file --fetch-schemes=vscode-webview,vscode-file --service-worker-schemes=vscode-webview --code-cache-schemes=vscode-webview,vscode-file --shared-files=v8_context_snapshot_data:100 --field-trial-handle=3,i,4472688112281498329,7132625848644244620,262144 --disable-features=CalculateNativeWinOcclusion,PlzDedicatedWorker,SpareRendererForSitePerProcess --variations-seed-version
PID: 6416 | Memory: 176.47 MB | CPU: 0.1% | /vscode/vscode-server/bin/linux-x64/fabdb6a30b49f79a7aba0f2ad9df9b399473380f/node /home/seoulrobotics/.vscode-server/extensions/streetsidesoftware.code-spell-checker-4.0.34/packages/_server/dist/main.cjs --node-ipc --clientProcessId=519
PID: 292625 | Memory: 105.63 MB | CPU: 0.0% | /nix/store/frchj7ynjnj5rwbhhdkwig25lz41y854-nodejs-20.11.1/bin/node /nix/store/sh0mn6qin1zakmcgyacrm26wbjs4fskx-corepack-nodejs-20.11.1/bin/yarn start


=== Node.js Processes Check: 2025-01-17 18:49:47 ===

Frontend Process:
PID: 292625 | Memory: 105.63 MB | CPU: 0.0% | /nix/store/frchj7ynjnj5rwbhhdkwig25lz41y854-nodejs-20.11.1/bin/node /nix/store/sh0mn6qin1zakmcgyacrm26wbjs4fskx-corepack-nodejs-20.11.1/bin/yarn start

Top Memory Node Processes:
PID: 5674 | Memory: 697.30 MB | CPU: 2.2% | /vscode/vscode-server/bin/linux-x64/fabdb6a30b49f79a7aba0f2ad9df9b399473380f/node --dns-result-order=ipv4first /vscode/vscode-server/bin/linux-x64/fabdb6a30b49f79a7aba0f2ad9df9b399473380f/out/bootstrap-fork --type=extensionHost --transformURIs --useHostProxy=true
PID: 42164 | Memory: 627.64 MB | CPU: 0.9% | /vscode/vscode-server/bin/linux-x64/fabdb6a30b49f79a7aba0f2ad9df9b399473380f/node --max-old-space-size=3072 /workspaces/horus/frontend/.yarn/sdks/typescript/lib/tsserver.js --useInferredProjectPerProjectRoot --enableTelemetry --cancellationPipeName /tmp/vscode-typescript1000/144cb6769dc94d06d1a5/tscancellation-a69fd01593a540a192b9.tmp* --globalPlugins typescript-deno-plugin,@vscode/copilot-typescript-server-plugin --pluginProbeLocations /home/seoulrobotics/.vscode-server/extensions/denoland.vscode-deno-3.43.2,/home/seoulrobotics/.vscode-server/extensions/github.copilot-chat-0.23.2 --locale en --noGetErrOnBackgroundUpdate --validateDefaultNpmLocation --useNodeIpc
PID: 184994 | Memory: 303.90 MB | CPU: 0.9% | /usr/share/code/code --type=utility --utility-sub-type=node.mojom.NodeService --lang=en-US --service-sandbox-type=none --dns-result-order=ipv4first --inspect-port=0 --crashpad-handler-pid=3924 --enable-crash-reporter=ecf269fb-51a8-4b4a-9839-b932a80428b9,no_channel --user-data-dir=/home/julian/.config/Code --standard-schemes=vscode-webview,vscode-file --enable-sandbox --secure-schemes=vscode-webview,vscode-file --cors-schemes=vscode-webview,vscode-file --fetch-schemes=vscode-webview,vscode-file --service-worker-schemes=vscode-webview --code-cache-schemes=vscode-webview,vscode-file --shared-files=v8_context_snapshot_data:100 --field-trial-handle=3,i,4472688112281498329,7132625848644244620,262144 --disable-features=CalculateNativeWinOcclusion,PlzDedicatedWorker,SpareRendererForSitePerProcess --variations-seed-version
PID: 6416 | Memory: 176.47 MB | CPU: 0.1% | /vscode/vscode-server/bin/linux-x64/fabdb6a30b49f79a7aba0f2ad9df9b399473380f/node /home/seoulrobotics/.vscode-server/extensions/streetsidesoftware.code-spell-checker-4.0.34/packages/_server/dist/main.cjs --node-ipc --clientProcessId=519
PID: 292625 | Memory: 105.63 MB | CPU: 0.0% | /nix/store/frchj7ynjnj5rwbhhdkwig25lz41y854-nodejs-20.11.1/bin/node /nix/store/sh0mn6qin1zakmcgyacrm26wbjs4fskx-corepack-nodejs-20.11.1/bin/yarn start

```
