//! This module implements the `message_type` getter for all protobuf message types.

use std::fmt;

use crate::protos::MessageType::*;
use crate::protos::*;

/// This trait extends the protobuf Message trait to also have a static getter for the message
/// type code.  This getter is implemented in this file for all the messages we use.
pub trait TrezorMessage: protobuf::Message + fmt::Debug {
	fn message_type() -> MessageType;
}

/// This macro provides the TrezorMessage trait for a protobuf message.
macro_rules! trezor_message_impl {
	($($struct:ident => $mtype:expr),+ $(,)?) => {$(
		impl TrezorMessage for $struct {
			#[inline]
			fn message_type() -> MessageType {
				$mtype
			}
		}
	)+};
}

trezor_message_impl! {
	// messages_common
	PinMatrixRequest => MessageType_PinMatrixRequest,
	PinMatrixAck => MessageType_PinMatrixAck,
	ButtonRequest => MessageType_ButtonRequest,
	ButtonAck => MessageType_ButtonAck,
	PassphraseRequest => MessageType_PassphraseRequest,
	PassphraseAck => MessageType_PassphraseAck,

	// messages_management
	Initialize => MessageType_Initialize,
	Ping => MessageType_Ping,
	Success => MessageType_Success,
	Failure => MessageType_Failure,
	ChangePin => MessageType_ChangePin,
	WipeDevice => MessageType_WipeDevice,
	GetEntropy => MessageType_GetEntropy,
	Entropy => MessageType_Entropy,
	LoadDevice => MessageType_LoadDevice,
	ResetDevice => MessageType_ResetDevice,
	Features => MessageType_Features,
	Cancel => MessageType_Cancel,
	ApplySettings => MessageType_ApplySettings,
	ApplyFlags => MessageType_ApplyFlags,
	BackupDevice => MessageType_BackupDevice,
	EntropyRequest => MessageType_EntropyRequest,
	EntropyAck => MessageType_EntropyAck,
	RecoveryDevice => MessageType_RecoveryDevice,
	WordRequest => MessageType_WordRequest,
	WordAck => MessageType_WordAck,
	GetFeatures => MessageType_GetFeatures,
	SetU2FCounter => MessageType_SetU2FCounter,

	// messages_bootloader
	FirmwareErase => MessageType_FirmwareErase,
	FirmwareUpload => MessageType_FirmwareUpload,
	FirmwareRequest => MessageType_FirmwareRequest,
	SelfTest => MessageType_SelfTest,

	// messages_crypto
	CipherKeyValue => MessageType_CipherKeyValue,
	CipheredKeyValue => MessageType_CipheredKeyValue,
	SignIdentity => MessageType_SignIdentity,
	SignedIdentity => MessageType_SignedIdentity,
	GetECDHSessionKey => MessageType_GetECDHSessionKey,
	ECDHSessionKey => MessageType_ECDHSessionKey,
	CosiCommit => MessageType_CosiCommit,
	CosiCommitment => MessageType_CosiCommitment,
	CosiSign => MessageType_CosiSign,
	CosiSignature => MessageType_CosiSignature,

	DebugLinkDecision => MessageType_DebugLinkDecision,
	DebugLinkGetState => MessageType_DebugLinkGetState,
	DebugLinkState => MessageType_DebugLinkState,
	DebugLinkStop => MessageType_DebugLinkStop,
	DebugLinkLog => MessageType_DebugLinkLog,
	DebugLinkMemoryRead => MessageType_DebugLinkMemoryRead,
	DebugLinkMemory => MessageType_DebugLinkMemory,
	DebugLinkMemoryWrite => MessageType_DebugLinkMemoryWrite,
	DebugLinkFlashErase => MessageType_DebugLinkFlashErase,
}

#[cfg(feature = "bitcoin")]
trezor_message_impl! {
	GetPublicKey => MessageType_GetPublicKey,
	PublicKey => MessageType_PublicKey,
	SignTx => MessageType_SignTx,
	TxRequest => MessageType_TxRequest,
	TxAck => MessageType_TxAck,
	GetAddress => MessageType_GetAddress,
	Address => MessageType_Address,
	SignMessage => MessageType_SignMessage,
	VerifyMessage => MessageType_VerifyMessage,
	MessageSignature => MessageType_MessageSignature,
}

#[cfg(feature = "ethereum")]
trezor_message_impl! {
	EthereumGetAddress => MessageType_EthereumGetAddress,
	EthereumAddress => MessageType_EthereumAddress,
	EthereumSignTx => MessageType_EthereumSignTx,
	EthereumSignTxEIP1559 => MessageType_EthereumSignTxEIP1559,
	EthereumTxRequest => MessageType_EthereumTxRequest,
	EthereumTxAck => MessageType_EthereumTxAck,
	EthereumSignMessage => MessageType_EthereumSignMessage,
	EthereumVerifyMessage => MessageType_EthereumVerifyMessage,
	EthereumMessageSignature => MessageType_EthereumMessageSignature,
	EthereumSignTypedData => MessageType_EthereumSignTypedData,
	EthereumTypedDataStructRequest => MessageType_EthereumTypedDataStructRequest,
	EthereumTypedDataStructAck => MessageType_EthereumTypedDataStructAck,
	EthereumTypedDataValueRequest => MessageType_EthereumTypedDataValueRequest,
	EthereumTypedDataValueAck => MessageType_EthereumTypedDataValueAck,
	EthereumTypedDataSignature => MessageType_EthereumTypedDataSignature,
}

#[cfg(feature = "nem")]
trezor_message_impl! {
	NEMGetAddress => MessageType_NEMGetAddress,
	NEMAddress => MessageType_NEMAddress,
	NEMSignTx => MessageType_NEMSignTx,
	NEMSignedTx => MessageType_NEMSignedTx,
	NEMDecryptMessage => MessageType_NEMDecryptMessage,
	NEMDecryptedMessage => MessageType_NEMDecryptedMessage,
}

#[cfg(feature = "lisk")]
trezor_message_impl! {
	LiskGetAddress => MessageType_LiskGetAddress,
	LiskAddress => MessageType_LiskAddress,
	LiskSignTx => MessageType_LiskSignTx,
	LiskSignedTx => MessageType_LiskSignedTx,
	LiskSignMessage => MessageType_LiskSignMessage,
	LiskMessageSignature => MessageType_LiskMessageSignature,
	LiskVerifyMessage => MessageType_LiskVerifyMessage,
	LiskGetPublicKey => MessageType_LiskGetPublicKey,
	LiskPublicKey => MessageType_LiskPublicKey,
}

#[cfg(feature = "tezos")]
trezor_message_impl! {
	TezosGetAddress => MessageType_TezosGetAddress,
	TezosAddress => MessageType_TezosAddress,
	TezosSignTx => MessageType_TezosSignTx,
	TezosSignedTx => MessageType_TezosSignedTx,
	TezosGetPublicKey => MessageType_TezosGetPublicKey,
	TezosPublicKey => MessageType_TezosPublicKey,
}

#[cfg(feature = "stellar")]
trezor_message_impl! {
	StellarSignTx => MessageType_StellarSignTx,
	StellarTxOpRequest => MessageType_StellarTxOpRequest,
	StellarGetAddress => MessageType_StellarGetAddress,
	StellarAddress => MessageType_StellarAddress,
	StellarCreateAccountOp => MessageType_StellarCreateAccountOp,
	StellarPaymentOp => MessageType_StellarPaymentOp,
	StellarPathPaymentOp => MessageType_StellarPathPaymentOp,
	StellarManageOfferOp => MessageType_StellarManageOfferOp,
	StellarCreatePassiveOfferOp => MessageType_StellarCreatePassiveOfferOp,
	StellarSetOptionsOp => MessageType_StellarSetOptionsOp,
	StellarChangeTrustOp => MessageType_StellarChangeTrustOp,
	StellarAllowTrustOp => MessageType_StellarAllowTrustOp,
	StellarAccountMergeOp => MessageType_StellarAccountMergeOp,
	StellarManageDataOp => MessageType_StellarManageDataOp,
	StellarBumpSequenceOp => MessageType_StellarBumpSequenceOp,
	StellarSignedTx => MessageType_StellarSignedTx,
}

#[cfg(feature = "tron")]
trezor_message_impl! {
	TronGetAddress => MessageType_TronGetAddress,
	TronAddress => MessageType_TronAddress,
	TronSignTx => MessageType_TronSignTx,
	TronSignedTx => MessageType_TronSignedTx,
}

#[cfg(feature = "cardano")]
trezor_message_impl! {
	CardanoSignTx => MessageType_CardanoSignTx,
	CardanoTxRequest => MessageType_CardanoTxRequest,
	CardanoGetPublicKey => MessageType_CardanoGetPublicKey,
	CardanoPublicKey => MessageType_CardanoPublicKey,
	CardanoGetAddress => MessageType_CardanoGetAddress,
	CardanoAddress => MessageType_CardanoAddress,
	CardanoTxAck => MessageType_CardanoTxAck,
	CardanoSignedTx => MessageType_CardanoSignedTx,
}

#[cfg(feature = "ontology")]
trezor_message_impl! {
	OntologyGetAddress => MessageType_OntologyGetAddress,
	OntologyAddress => MessageType_OntologyAddress,
	OntologyGetPublicKey => MessageType_OntologyGetPublicKey,
	OntologyPublicKey => MessageType_OntologyPublicKey,
	OntologySignTransfer => MessageType_OntologySignTransfer,
	OntologySignedTransfer => MessageType_OntologySignedTransfer,
	OntologySignWithdrawOng => MessageType_OntologySignWithdrawOng,
	OntologySignedWithdrawOng => MessageType_OntologySignedWithdrawOng,
	OntologySignOntIdRegister => MessageType_OntologySignOntIdRegister,
	OntologySignedOntIdRegister => MessageType_OntologySignedOntIdRegister,
	OntologySignOntIdAddAttributes => MessageType_OntologySignOntIdAddAttributes,
	OntologySignedOntIdAddAttributes => MessageType_OntologySignedOntIdAddAttributes,
}

#[cfg(feature = "ripple")]
trezor_message_impl! {
	RippleGetAddress => MessageType_RippleGetAddress,
	RippleAddress => MessageType_RippleAddress,
	RippleSignTx => MessageType_RippleSignTx,
	RippleSignedTx => MessageType_RippleSignedTx,
}

#[cfg(feature = "monero")]
trezor_message_impl! {
	MoneroTransactionInitRequest => MessageType_MoneroTransactionInitRequest,
	MoneroTransactionInitAck => MessageType_MoneroTransactionInitAck,
	MoneroTransactionSetInputRequest => MessageType_MoneroTransactionSetInputRequest,
	MoneroTransactionSetInputAck => MessageType_MoneroTransactionSetInputAck,
	MoneroTransactionInputsPermutationRequest => MessageType_MoneroTransactionInputsPermutationRequest,
	MoneroTransactionInputsPermutationAck => MessageType_MoneroTransactionInputsPermutationAck,
	MoneroTransactionInputViniRequest => MessageType_MoneroTransactionInputViniRequest,
	MoneroTransactionInputViniAck => MessageType_MoneroTransactionInputViniAck,
	MoneroTransactionAllInputsSetRequest => MessageType_MoneroTransactionAllInputsSetRequest,
	MoneroTransactionAllInputsSetAck => MessageType_MoneroTransactionAllInputsSetAck,
	MoneroTransactionSetOutputRequest => MessageType_MoneroTransactionSetOutputRequest,
	MoneroTransactionSetOutputAck => MessageType_MoneroTransactionSetOutputAck,
	MoneroTransactionAllOutSetRequest => MessageType_MoneroTransactionAllOutSetRequest,
	MoneroTransactionAllOutSetAck => MessageType_MoneroTransactionAllOutSetAck,
	MoneroTransactionMlsagDoneRequest => MessageType_MoneroTransactionMlsagDoneRequest,
	MoneroTransactionMlsagDoneAck => MessageType_MoneroTransactionMlsagDoneAck,
	MoneroTransactionSignInputRequest => MessageType_MoneroTransactionSignInputRequest,
	MoneroTransactionSignInputAck => MessageType_MoneroTransactionSignInputAck,
	MoneroTransactionFinalRequest => MessageType_MoneroTransactionFinalRequest,
	MoneroTransactionFinalAck => MessageType_MoneroTransactionFinalAck,
	MoneroKeyImageExportInitRequest => MessageType_MoneroKeyImageExportInitRequest,
	MoneroKeyImageExportInitAck => MessageType_MoneroKeyImageExportInitAck,
	MoneroKeyImageSyncStepRequest => MessageType_MoneroKeyImageSyncStepRequest,
	MoneroKeyImageSyncStepAck => MessageType_MoneroKeyImageSyncStepAck,
	MoneroKeyImageSyncFinalRequest => MessageType_MoneroKeyImageSyncFinalRequest,
	MoneroKeyImageSyncFinalAck => MessageType_MoneroKeyImageSyncFinalAck,
	MoneroGetAddress => MessageType_MoneroGetAddress,
	MoneroAddress => MessageType_MoneroAddress,
	MoneroGetWatchKey => MessageType_MoneroGetWatchKey,
	MoneroWatchKey => MessageType_MoneroWatchKey,
	DebugMoneroDiagRequest => MessageType_DebugMoneroDiagRequest,
	DebugMoneroDiagAck => MessageType_DebugMoneroDiagAck,
}
