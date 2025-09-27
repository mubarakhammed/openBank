-- Create transaction status enum
CREATE TYPE transaction_status AS ENUM ('pending', 'completed', 'failed', 'cancelled');

-- Create transaction type enum
CREATE TYPE transaction_type AS ENUM ('deposit', 'withdrawal', 'transfer', 'payment', 'refund');

-- Create transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_account_id UUID REFERENCES accounts(id),
    to_account_id UUID REFERENCES accounts(id),
    amount BIGINT NOT NULL CHECK (amount > 0),
    currency VARCHAR(3) DEFAULT 'USD',
    transaction_type transaction_type NOT NULL,
    status transaction_status DEFAULT 'pending',
    reference VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_transactions_from_account ON transactions(from_account_id);
CREATE INDEX IF NOT EXISTS idx_transactions_to_account ON transactions(to_account_id);
CREATE INDEX IF NOT EXISTS idx_transactions_reference ON transactions(reference);
CREATE INDEX IF NOT EXISTS idx_transactions_status ON transactions(status);
CREATE INDEX IF NOT EXISTS idx_transactions_created_at ON transactions(created_at);