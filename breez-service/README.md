# Breez SDK Microservice

Node.js service that wraps the Breez SDK Spark (Nodeless) implementation.
This avoids the Rust sqlite linking conflict by running Breez SDK in a separate process.

## Setup

```bash
cd breez-service
npm install
```

## Run

```bash
# Set environment variables
set BREEZ_API_KEY=MIIBczCCASWgAwIBAgIHPq+GoWjQ1zAFBgMrZXAwEDEOMAwGA1UEAxMFQnJlZXowHhcNMjUxMTI5MTkyMjEyWhcNMzUxMTI3MTkyMjEyWjAvMRQwEgYDVQQKEwtTYWJpIFdhbGxldDEXMBUGA1UEAxMOQXV3YWwgQWJ1YmFrYXIwKjAFBgMrZXADIQDQg/XL3yA8HKIgyimHU/Qbpxy0tvzris1fDUtEs6ldd6N/MH0wDgYDVR0PAQH/BAQDAgWgMAwGA1UdEwEB/wQCMAAwHQYDVR0OBBYEFNo5o+5ea0sNMlW/75VgGJCv2AcJMB8GA1UdIwQYMBaAFN6q1pJW843ndJIW/Ey2ILJrKJhrMB0GA1UdEQQWMBSBEmF1d2Fscmc4QGdtYWlsLmNvbTAFBgMrZXADQQCInVRb1DyioxmjSLOhYLggfLiO1wXyTWRMEh5PhU5a8M0lWteV7hmQvjJr9SN3I+JVutSWGlnu5tgz3bRQJHAN
set BREEZ_SERVICE_PORT=3001

npm start
```

## Endpoints

### POST /api/create-node
Creates a new Breez Spark node.

Request:
```json
{
  "wallet_id": "uuid-v7-string"
}
```

Response:
```json
{
  "node_id": "02abc...",
  "invite_code": "word word word...",
  "status": "created"
}
```

### POST /api/open-channel
Opens a Lightning channel with inbound liquidity.

Request:
```json
{
  "wallet_id": "uuid-v7-string",
  "amount_sats": 200000
}
```

Response:
```json
{
  "success": true,
  "bitcoin_address": "bc1q...",
  "swap_fee_sat": 1000,
  "message": "Send BTC to this address..."
}
```

### GET /health
Health check.

Response:
```json
{
  "status": "ok",
  "service": "sabi-breez-service",
  "active_wallets": 0
}
```
