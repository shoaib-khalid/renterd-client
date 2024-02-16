# renterd-client
A simple sia network renterd client written in rust.

Uses the renterd API: https://api.sia.tech/renterd, to connect to the sia network.

Only implements two endpoints of the renterd API:

1. localhost:9980/api/bus/state (https://api.sia.tech/renterd#f41d143e-3cc7-4a67-bae6-ad308489fecc)
2. localhost:9980/api/worker/objects/:key (https://api.sia.tech/renterd#4978ac58-f401-49e3-8ec1-4d5d32ea5f72)
