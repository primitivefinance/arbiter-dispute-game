#[macro_export]
macro_rules! tx {
    ($to:expr, $data:expr) => {{
        use ethers::types::{
            transaction::eip2718::TypedTransaction, Eip1559TransactionRequest,
            NameOrAddress, H160,
        };

        TypedTransaction::Eip1559(Eip1559TransactionRequest {
            to: Some(NameOrAddress::Address(H160::from_slice($to.as_slice()))),
            data: Some($data),
            ..Default::default()
        })
    }};
    ($to:expr, $data:expr, $value:expr) => {{
        use ethers::types::{
            transaction::eip2718::TypedTransaction, Eip1559TransactionRequest,
            NameOrAddress, H160,
        };

        TypedTransaction::Eip1559(Eip1559TransactionRequest {
            to: Some(NameOrAddress::Address(H160::from_slice($to.as_slice()))),
            data: Some($data),
            value
            ..Default::default()
        })
    }};
}

#[macro_export]
macro_rules! send_tx {
    ($agent:expr, $to:expr, $data:expr) => {{
        async {
            let transaction = tx!($to, $data);
            let receipt = $agent
                .client
                .send_transaction(transaction, None)
                .await?
                .await?
                .ok_or(anyhow!("No receipt for transaction"))?;
            Ok::<_, anyhow::Error>(receipt)
        }
    }};
    ($agent:expr, $to:expr, $data:expr, $value:expr) => {{
        async {
            let transaction = tx!($to, $data, $value);
            let receipt = $agent
                .client
                .send_transaction(transaction, None)
                .await?
                .await?
                .ok_or(anyhow!("No receipt for transaction"))?;
            Ok::<_, anyhow::Error>(receipt)
        }
    }};
}
