require('dotenv').config();
const express = require('express');
const bip39 = require('bip39');
const breezSdk = require('@breeztech/breez-sdk-spark');

const app = express();
app.use(express.json());

const PORT = process.env.BREEZ_SERVICE_PORT || 3001;
const BREEZ_API_KEY = process.env.BREEZ_API_KEY;

if (!BREEZ_API_KEY) {
  console.error('ERROR: BREEZ_API_KEY environment variable is required');
  process.exit(1);
}

// Store SDK instances by wallet_id (in-memory for now)
const sdkInstances = new Map();

/**
 * POST /api/create-node
 * Body: { wallet_id: string }
 * Creates a new Breez Spark node and returns invite_code and node_id
 */
app.post('/api/create-node', async (req, res) => {
  try {
    const { wallet_id } = req.body;
    
    if (!wallet_id) {
      return res.status(400).json({ error: 'wallet_id is required' });
    }

    console.log(`Creating Breez node for wallet_id: ${wallet_id}`);

    // Generate BIP-39 mnemonic (24 words for production)
    const mnemonic = bip39.generateMnemonic(256); // 24 words
    
    // Create seed from mnemonic
    const seed = {
      type: 'mnemonic',
      mnemonic,
      passphrase: null
    };

    // Configure Breez SDK for mainnet
    const config = breezSdk.defaultConfig(breezSdk.Network.Mainnet);
    config.apiKey = BREEZ_API_KEY;

    // Connect to Breez Spark
    const storageDir = `./data/breez/${wallet_id}`;
    const sdk = await breezSdk.connect({
      config,
      seed,
      storageDir
    });

    // Get node info
    const nodeInfo = await sdk.nodeInfo();

    // Store SDK instance for later use (channel opening, payments, etc.)
    sdkInstances.set(wallet_id, { sdk, mnemonic });

    console.log(`✅ Breez node created: ${nodeInfo.id}`);

    res.json({
      node_id: nodeInfo.id,
      invite_code: mnemonic, // Return mnemonic as invite_code for recovery
      status: 'created'
    });

  } catch (error) {
    console.error('Error creating Breez node:', error);
    res.status(500).json({ 
      error: 'Failed to create Breez node',
      details: error.message 
    });
  }
});

/**
 * POST /api/open-channel
 * Body: { wallet_id: string, amount_sats: number }
 * Provisions inbound liquidity via Breez LSP
 */
app.post('/api/open-channel', async (req, res) => {
  try {
    const { wallet_id, amount_sats } = req.body;

    if (!wallet_id || !amount_sats) {
      return res.status(400).json({ 
        error: 'wallet_id and amount_sats are required' 
      });
    }

    // Validate amount
    if (amount_sats < 100000 || amount_sats > 300000) {
      return res.status(400).json({ 
        error: 'amount_sats must be between 100000 and 300000' 
      });
    }

    console.log(`Opening channel for wallet ${wallet_id}: ${amount_sats} sats`);

    const instance = sdkInstances.get(wallet_id);
    if (!instance) {
      return res.status(404).json({ 
        error: 'Wallet not found. Create node first.' 
      });
    }

    const { sdk } = instance;

    // Request inbound liquidity via receive_onchain
    // This creates a submarine swap that provisions channel capacity
    const receiveRequest = {
      amountSat: amount_sats,
    };

    const swapInfo = await sdk.receiveOnchain(receiveRequest);

    console.log(`✅ Channel opening initiated. Pay to: ${swapInfo.bitcoinAddress}`);

    res.json({
      success: true,
      bitcoin_address: swapInfo.bitcoinAddress,
      min_allowed_deposit: swapInfo.minAllowedDeposit,
      max_allowed_deposit: swapInfo.maxAllowedDeposit,
      swap_fee_sat: swapInfo.totalFeesSat,
      message: 'Send BTC to this address to open channel with inbound liquidity'
    });

  } catch (error) {
    console.error('Error opening channel:', error);
    res.status(500).json({ 
      error: 'Failed to open channel',
      details: error.message 
    });
  }
});

/**
 * GET /health
 * Health check endpoint
 */
app.get('/health', (req, res) => {
  res.json({ 
    status: 'ok',
    service: 'sabi-breez-service',
    active_wallets: sdkInstances.size
  });
});

app.listen(PORT, () => {
  console.log(`🚀 Breez SDK microservice listening on http://localhost:${PORT}`);
  console.log(`📡 Using Breez API key: ${BREEZ_API_KEY.substring(0, 20)}...`);
});
