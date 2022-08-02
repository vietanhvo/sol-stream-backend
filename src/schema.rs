table! {
    streams (pda_account) {
        pda_account -> Varchar,
        start_time -> Bigint,
        end_time -> Bigint,
        receiver -> Varchar,
        lamports_withdrawn -> Bigint,
        amount_second -> Bigint,
        sender -> Varchar,
        total_amount -> Bigint,
    }
}
