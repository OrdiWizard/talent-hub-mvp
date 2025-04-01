const express = require('express');
const connectDB = require('./database');
const authRoutes = require('./routes/authRoutes');
const escrowRoutes = require('./routes/escrowRoutes');

const app = express();
app.use(express.json());

connectDB();

app.use('/api/auth', authRoutes);
app.use('/api/escrow', escrowRoutes);

app.listen(5000, () => {
    console.log('Server running on port 5000');
});
