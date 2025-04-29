#![no_main]
#![no_std]
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use simplealloc::SimpleAlloc;

#[global_allocator]
pub static mut GLOBAL: SimpleAlloc<{ 1024 * 10 }> = SimpleAlloc::new();

use uapi::{input, HostFn, HostFnImpl as api, ReturnFlags, StorageFlags};

use ethabi::{decode, ethereum_types::U256, ParamType, Token};

const NAME: &[u8] = b"name";
const SYMBOL: &[u8] = b"symbol";
const NAME_LENGTH: &[u8] = b"name_length";
const SYMBOL_LENGTH: &[u8] = b"symbol_length";

const DECIMALS: &[u8] = b"decimals";
const TOTAL_SUPPLY: &[u8] = b"total_supply";

const BALANCE: &[u8] = b"balance";
const ALLOWANCE: &[u8] = b"allowance";

// Define constants for all your function selectors
const NAME_SELECTOR: [u8; 4] = [0x06, 0xfd, 0xde, 0x03]; // name()
const SYMBOL_SELECTOR: [u8; 4] = [0x95, 0xd8, 0x9b, 0x41]; // symbol()
const DECIMALS_SELECTOR: [u8; 4] = [0x31, 0x3c, 0xe5, 0x67]; // decimals()
const TOTAL_SUPPLY_SELECTOR: [u8; 4] = [0x18, 0x16, 0x0d, 0xdd]; // totalSupply()
const BALANCE_OF_SELECTOR: [u8; 4] = [0x70, 0xa0, 0x82, 0x31]; // balanceOf(address)
const TRANSFER_SELECTOR: [u8; 4] = [0xa9, 0x05, 0x9c, 0xbb]; // transfer(address,uint256)
const APPROVE_SELECTOR: [u8; 4] = [0x09, 0x5e, 0xa7, 0xb3]; // approve(address,uint256)
const ALLOWANCE_SELECTOR: [u8; 4] = [0xdd, 0x62, 0xed, 0x3e]; // allowance(address,address)
const TRANSFER_FROM_SELECTOR: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd]; // transferFrom(address,address,uint256)

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        core::arch::asm!("unimp");
        core::hint::unreachable_unchecked();
    }
}

#[no_mangle]
#[polkavm_derive::polkavm_export]
pub extern "C" fn deploy() {
    input!(data: &[u8; 256],);
    let mut sender = [0_u8; 20];
    api::origin(&mut sender);

    let param_types = &[
        ParamType::String,
        ParamType::String,
        ParamType::Uint(256),
        ParamType::Uint(256),
    ];

    let decode_result = decode(param_types, &data[..]).unwrap();

    if let (
        Token::String(name),
        Token::String(symbol),
        Token::Uint(decimals),
        Token::Uint(total_supply),
    ) = (
        &decode_result[0],
        &decode_result[1],
        &decode_result[2],
        &decode_result[3],
    ) {
        // api::set_storage(StorageFlags::empty(), NAME, &name.as_bytes());
        // set_name(name.len() as u16, &name.as_bytes());
        // api::set_storage(StorageFlags::empty(), SYMBOL, &symbol.as_bytes()[..]);
        set_string(NAME_LENGTH, NAME, name.as_bytes());
        set_string(SYMBOL_LENGTH, SYMBOL, symbol.as_bytes());
        let mut data = [0_u8; 32];
        decimals.to_big_endian(&mut data);

        api::set_storage(StorageFlags::empty(), DECIMALS, &data);

        let supply = U256::from(10).pow(*decimals).saturating_mul(*total_supply);
        supply.to_big_endian(&mut data);
        api::set_storage(StorageFlags::empty(), TOTAL_SUPPLY, &data);
        api::set_storage(StorageFlags::empty(), &get_balance_key(&sender), &data);
    } else {
        panic!("Failed to decode input data");
    }
}

/// This is the regular entry point when the contract is called.
#[no_mangle]
#[polkavm_derive::polkavm_export]
pub extern "C" fn call() {
    input!(selector: &[u8; 4],);
    let length = api::call_data_size();
    if length > 256 {
        panic!("Input data too long");
    }
    let mut sender = [0_u8; 20];
    api::origin(&mut sender);

    match selector {
        &NAME_SELECTOR => {
            api::return_value(ReturnFlags::empty(), &get_string(NAME_LENGTH, NAME)[..])
        }
        &SYMBOL_SELECTOR => {
            api::return_value(ReturnFlags::empty(), &get_string(SYMBOL_LENGTH, SYMBOL)[..])
        }
        &DECIMALS_SELECTOR => {
            let mut data = [0_u8; 32];
            let _ = api::get_storage(StorageFlags::empty(), DECIMALS, &mut &mut data[..]);
            api::return_value(ReturnFlags::empty(), &data[..])
        }
        &TOTAL_SUPPLY_SELECTOR => {
            let mut data = [0_u8; 32];
            let _ = api::get_storage(StorageFlags::empty(), TOTAL_SUPPLY, &mut &mut data[..]);
            api::return_value(ReturnFlags::empty(), &data[..])
        }
        &BALANCE_OF_SELECTOR => {
            input!(buffer: &[u8; 4 + 32],);
            let param_types = &[ParamType::Address];
            let decode_result = decode(param_types, &buffer[4..]).unwrap();

            if let Token::Address(address) = &decode_result[0] {
                let mut data = [0_u8; 32];
                let _ = api::get_storage(
                    StorageFlags::empty(),
                    &get_balance_key(&address.to_fixed_bytes()),
                    &mut &mut data[..],
                );
                api::return_value(ReturnFlags::empty(), &data[..])
            } else {
                panic!("Failed to decode input data");
            }
        }
        &ALLOWANCE_SELECTOR => {
            input!(buffer: &[u8; 4 + 32 + 32],);
            let param_types = &[ParamType::Address, ParamType::Address];
            let decode_result = decode(param_types, &buffer[4..]).unwrap();

            if let (Token::Address(from), Token::Address(spender)) =
                (&decode_result[0], &decode_result[1])
            {
                let mut data = [0_u8; 32];
                let _ = api::get_storage(
                    StorageFlags::empty(),
                    &get_allownance_key(&from.to_fixed_bytes(), &spender.to_fixed_bytes()),
                    &mut &mut data[..],
                );
                api::return_value(ReturnFlags::empty(), &data[..])
            } else {
                panic!("Failed to decode input data");
            }
        }
        &TRANSFER_SELECTOR => {
            input!(buffer: &[u8; 4 + 32 + 32],);
            let param_types = &[ParamType::Address, ParamType::Uint(256)];
            let decode_result = decode(param_types, &buffer[4..]).unwrap();

            if let (Token::Address(recipient), Token::Uint(amount)) =
                (&decode_result[0], &decode_result[1])
            {
                let mut data = [0_u8; 32];

                let _ = api::get_storage(
                    StorageFlags::empty(),
                    &get_balance_key(&sender),
                    &mut &mut data[..],
                );
                let sender_balance = U256::from_big_endian(&data);
                if sender_balance < *amount {
                    panic!("Insufficient balance");
                }

                data = [0_u8; 32];
                let _ = api::get_storage(
                    StorageFlags::empty(),
                    &get_balance_key(&recipient.to_fixed_bytes()),
                    &mut &mut data[..],
                );
                let recipient_balance = U256::from_big_endian(&data);

                data = [0_u8; 32];
                sender_balance
                    .saturating_sub(*amount)
                    .to_big_endian(&mut data);

                api::set_storage(StorageFlags::empty(), &get_balance_key(&sender), &data);

                data = [0_u8; 32];
                recipient_balance
                    .saturating_add(*amount)
                    .to_big_endian(&mut data);
                api::set_storage(
                    StorageFlags::empty(),
                    &get_balance_key(&recipient.to_fixed_bytes()),
                    &data,
                );

                api::return_value(ReturnFlags::empty(), &[0_u8; 32])
            } else {
                panic!("Failed to decode input data");
            }
        }
        &APPROVE_SELECTOR => {
            input!(buffer: &[u8; 4 + 32 + 32],);
            let param_types = &[ParamType::Address, ParamType::Uint(256)];
            let decode_result = decode(param_types, &buffer[4..]).unwrap();

            if let (Token::Address(spender), Token::Uint(amount)) =
                (&decode_result[0], &decode_result[1])
            {
                let mut data = [0_u8; 32];

                let _ = api::get_storage(
                    StorageFlags::empty(),
                    &get_balance_key(&sender),
                    &mut &mut data[..],
                );
                let sender_balance = U256::from_big_endian(&data);
                if sender_balance < *amount {
                    panic!("Insufficient balance");
                }

                data = [0_u8; 32];
                let _ = api::get_storage(
                    StorageFlags::empty(),
                    &get_allownance_key(&sender, &spender.to_fixed_bytes()),
                    &mut &mut data[..],
                );
                let allowance = U256::from_big_endian(&data);

                data = [0_u8; 32];
                sender_balance
                    .saturating_sub(*amount)
                    .to_big_endian(&mut data);

                api::set_storage(StorageFlags::empty(), &get_balance_key(&sender), &data);

                data = [0_u8; 32];
                allowance.saturating_add(*amount).to_big_endian(&mut data);
                api::set_storage(
                    StorageFlags::empty(),
                    &get_allownance_key(&sender, &spender.to_fixed_bytes()),
                    &data,
                );

                api::return_value(ReturnFlags::empty(), &[0_u8; 32])
            } else {
                panic!("Failed to decode input data");
            }
        }
        &TRANSFER_FROM_SELECTOR => {
            input!(buffer: &[u8; 4 + 32 + 32 + 32],);
            let param_types = &[ParamType::Address, ParamType::Address, ParamType::Uint(256)];
            let decode_result = decode(param_types, &buffer[4..]).unwrap();

            if let (Token::Address(from), Token::Address(recipient), Token::Uint(amount)) =
                (&decode_result[0], &decode_result[1], &decode_result[2])
            {
                let mut data = [0_u8; 32];

                let _ = api::get_storage(
                    StorageFlags::empty(),
                    &get_allownance_key(&from.to_fixed_bytes(), &sender),
                    &mut &mut data[..],
                );
                let current_allowance = U256::from_big_endian(&data);
                if current_allowance < *amount {
                    panic!("Insufficient balance");
                }

                data = [0_u8; 32];
                let _ = api::get_storage(
                    StorageFlags::empty(),
                    &get_allownance_key(&sender, &recipient.to_fixed_bytes()),
                    &mut &mut data[..],
                );
                let recipient_balance = U256::from_big_endian(&data);

                data = [0_u8; 32];
                current_allowance
                    .saturating_sub(*amount)
                    .to_big_endian(&mut data);

                api::set_storage(
                    StorageFlags::empty(),
                    &get_allownance_key(&from.to_fixed_bytes(), &sender),
                    &data,
                );

                data = [0_u8; 32];
                recipient_balance
                    .saturating_add(*amount)
                    .to_big_endian(&mut data);
                api::set_storage(
                    StorageFlags::empty(),
                    &get_balance_key(&recipient.to_fixed_bytes()),
                    &data,
                );

                api::return_value(ReturnFlags::empty(), &[0_u8; 32])
            } else {
                panic!("Failed to decode input data");
            }
        }
        _ => panic!("Unknown function"),
    }
}

pub fn get_balance_key(sender: &[u8; 20]) -> [u8; 27] {
    let mut key = [0u8; 27];
    key[0..7].copy_from_slice(BALANCE);
    key[7..27].copy_from_slice(sender);
    key
}

pub fn get_allownance_key(sender: &[u8; 20], spender: &[u8; 20]) -> [u8; 49] {
    let mut key = [0u8; 49];
    key[0..9].copy_from_slice(ALLOWANCE);
    key[9..29].copy_from_slice(sender);
    key[29..49].copy_from_slice(spender);
    key
}

pub fn get_string(length_key: &[u8], data_key: &[u8]) -> Vec<u8> {
    // max length is 256 * 256
    let mut length_bytes = [0u8; 2];
    let _ = api::get_storage(
        uapi::StorageFlags::empty(),
        length_key,
        &mut &mut length_bytes[..],
    );
    let length = u16::from_be_bytes(length_bytes);
    // read data according to the length
    let mut data = vec![0u8; length as usize];
    let _ = api::get_storage(uapi::StorageFlags::empty(), data_key, &mut &mut data[..]);
    // compute padding length
    let result_length = if length % 32 == 0 {
        length + 64
    } else {
        (length / 32 + 1) * 32 + 64
    };
    let mut result = vec![0u8; result_length as usize];
    // set index of string length
    result[31] = 0x20;
    // set length
    result[62..64].copy_from_slice(&length_bytes);
    result[64..(64 + data.len())].copy_from_slice(&data[..]);
    result
}

pub fn set_string(length_key: &[u8], data_key: &[u8], data: &[u8]) {
    let length = data.len() as u16;
    let _ = api::set_storage(
        uapi::StorageFlags::empty(),
        length_key,
        &length.to_be_bytes(),
    );
    api::set_storage(uapi::StorageFlags::empty(), data_key, &data[..]);
}
