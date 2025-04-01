const express = require('express');
const { createEscrow } = require('../blockchain/solana');

const router = express.Router();

router.post('/create', async (req, res) => {
    try {
        const { employer, freelancer, amount } = req.body;
        const escrowAddress = await createEscrow(employer, freelancer, amount);
        res.json({ escrowAddress });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

module.exports = router;
