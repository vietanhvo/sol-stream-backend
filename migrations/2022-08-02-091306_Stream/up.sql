-- Your SQL goes here
Create Table streams(
    pda_account VARCHAR(44),
    start_time BIGINT NOT NULL,
    end_time BIGINT NOT NULL,
    receiver VARCHAR(44) NOT NULL,
    lamports_withdrawn BIGINT NOT NULL,
    amount_second BIGINT NOT NULL,
    sender VARCHAR(44) NOT NULL,
    total_amount BIGINT NOT NULL,
    PRIMARY KEY (pda_account)
)
