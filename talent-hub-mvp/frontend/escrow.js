import { useState } from 'react';

export default function Escrow() {
    const [freelancer, setFreelancer] = useState('');
    const [amount, setAmount] = useState('');

    const handleCreateEscrow = async () => {
        const res = await fetch('/api/escrow/create', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ employer: "ТВІЙ_SOLANA_WALLET", freelancer, amount }),
        });
        const data = await res.json();
        alert(`Escrow Created: ${data.escrowAddress}`);
    };

    return (
        <div>
            <h2>Create Escrow</h2>
            <input type="text" placeholder="Freelancer Wallet" value={freelancer} onChange={(e) => setFreelancer(e.target.value)} />
            <input type="number" placeholder="Amount" value={amount} onChange={(e) => setAmount(e.target.value)} />
            <button onClick={handleCreateEscrow}>Create</button>
        </div>
    );
}
