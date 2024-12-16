/// ICP Token Wallet Implementation
/// Implements IRCRC2 token standard for Internet Computer Protocol
use candid::{CandidType, Deserialize};
use ic_cdk_macros::{query, update};
use ic_cdk::caller;
use std::cell::RefCell;

/// Token structure representing ownership and balance
#[derive(CandidType, Deserialize, Clone)]
struct Token {
    owner: String,
    balance: u64,
}

/// Global state for token storage
thread_local! {
    static TOKENS: RefCell<Vec<Token>> = RefCell::new(Vec::new());
}

/// Send tokens to another address
/// Returns Result indicating success or failure with error message
#[update]
fn send_tokens(to: String, amount: u64) -> Result<(), String> {
    TOKENS.with(|tokens| {
        let mut tokens = tokens.borrow_mut();
        let sender = caller().to_text();
        
        let sender_token = tokens.iter_mut().find(|t| t.owner == sender);
        if let Some(sender_token) = sender_token {
            if sender_token.balance < amount {
                return Err("Insufficient balance".to_string());
            }
            sender_token.balance -= amount;
        } else {
            return Err("Sender not found".to_string());
        }

        if let Some(receiver_token) = tokens.iter_mut().find(|t| t.owner == to) {
            receiver_token.balance += amount;
        } else {
            tokens.push(Token {
                owner: to,
                balance: amount,
            });
        }

        Ok(())
    })
}

/// Receive tokens from another address
/// Updates balance for the receiving address
#[update]
fn receive_tokens(_from: String, amount: u64) {
    TOKENS.with(|tokens| {
        let mut tokens = tokens.borrow_mut();
        let receiver = caller().to_text();
        
        if let Some(token) = tokens.iter_mut().find(|t| t.owner == receiver) {
            token.balance += amount;
        } else {
            tokens.push(Token {
                owner: receiver,
                balance: amount,
            });
        }
    });
}

/// Query current balance for caller
#[query]
fn get_balance() -> u64 {
    TOKENS.with(|tokens| {
        let tokens = tokens.borrow();
        let owner = caller().to_text();
        tokens.iter().find(|t| t.owner == owner).map_or(0, |t| t.balance)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk_test::MockContext;

    #[test]
    fn test_send_tokens() {
        let ctx = MockContext::new();
        let sender = "sender".to_string();
        ctx.set_caller(sender.clone());

        // Initialize sender balance
        TOKENS.with(|tokens| {
            tokens.borrow_mut().push(Token {
                owner: sender.clone(),
                balance: 100,
            });
        });

        // Test sending tokens
        let result = send_tokens("receiver".to_string(), 50);
        assert!(result.is_ok());

        // Verify balances
        TOKENS.with(|tokens| {
            let tokens = tokens.borrow();
            assert_eq!(tokens.iter().find(|t| t.owner == sender).unwrap().balance, 50);
            assert_eq!(tokens.iter().find(|t| t.owner == "receiver").unwrap().balance, 50);
        });
    }

    #[test]
    fn test_insufficient_balance() {
        let ctx = MockContext::new();
        let sender = "sender".to_string();
        ctx.set_caller(sender.clone());

        // Initialize sender balance
        TOKENS.with(|tokens| {
            tokens.borrow_mut().push(Token {
                owner: sender.clone(),
                balance: 40,
            });
        });

        // Test sending more tokens than available
        let result = send_tokens("receiver".to_string(), 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_balance() {
        let ctx = MockContext::new();
        let owner = "owner".to_string();
        ctx.set_caller(owner.clone());

        // Initialize balance
        TOKENS.with(|tokens| {
            tokens.borrow_mut().push(Token {
                owner: owner.clone(),
                balance: 100,
            });
        });

        assert_eq!(get_balance(), 100);
    }
}