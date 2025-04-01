import { useEffect, useState } from 'react';

export default function Dashboard() {
    const [user, setUser] = useState(null);

    useEffect(() => {
        const token = localStorage.getItem('token');
        if (!token) {
            window.location.href = '/login';
            return;
        }
        
        fetch('/api/auth/me', {
            headers: { 'Authorization': `Bearer ${token}` }
        })
        .then(res => res.json())
        .then(data => setUser(data));
    }, []);

    return (
        <div>
            <h2>Dashboard</h2>
            {user ? <p>Welcome, {user.email}</p> : <p>Loading...</p>}
        </div>
    );
}
