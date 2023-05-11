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
	($struct:ident, $mtype:expr) => {
		impl TrezorMessage for $struct {
			fn message_type() -> MessageType {
				$mtype
			}
		}
	};
}

trezor_message_impl!(Initialize, MessageType_Initialize);
trezor_message_impl!(Ping, MessageType_Ping);
trezor_message_impl!(Success, MessageType_Success);
trezor_message_impl!(Failure, MessageType_Failure);
trezor_message_impl!(ChangePin, MessageType_ChangePin);
trezor_message_impl!(WipeDevice, MessageType_WipeDevice);
trezor_message_impl!(GetEntropy, MessageType_GetEntropy);
trezor_message_impl!(Entropy, MessageType_Entropy);
trezor_message_impl!(LoadDevice, MessageType_LoadDevice);
trezor_message_impl!(ResetDevice, MessageType_ResetDevice);
trezor_message_impl!(Features, MessageType_Features);
trezor_message_impl!(PinMatrixRequest, MessageType_PinMatrixRequest);
trezor_message_impl!(PinMatrixAck, MessageType_PinMatrixAck);
trezor_message_impl!(Cancel, MessageType_Cancel);
trezor_message_impl!(ApplySettings, MessageType_ApplySettings);
trezor_message_impl!(ButtonRequest, MessageType_ButtonRequest);
trezor_message_impl!(ButtonAck, MessageType_ButtonAck);
trezor_message_impl!(ApplyFlags, MessageType_ApplyFlags);
trezor_message_impl!(BackupDevice, MessageType_BackupDevice);
trezor_message_impl!(EntropyRequest, MessageType_EntropyRequest);
trezor_message_impl!(EntropyAck, MessageType_EntropyAck);
trezor_message_impl!(PassphraseRequest, MessageType_PassphraseRequest);
trezor_message_impl!(PassphraseAck, MessageType_PassphraseAck);
trezor_message_impl!(RecoveryDevice, MessageType_RecoveryDevice);
trezor_message_impl!(WordRequest, MessageType_WordRequest);
trezor_message_impl!(WordAck, MessageType_WordAck);
trezor_message_impl!(GetFeatures, MessageType_GetFeatures);
trezor_message_impl!(SetU2FCounter, MessageType_SetU2FCounter);
trezor_message_impl!(FirmwareErase, MessageType_FirmwareErase);
trezor_message_impl!(FirmwareUpload, MessageType_FirmwareUpload);
trezor_message_impl!(FirmwareRequest, MessageType_FirmwareRequest);
trezor_message_impl!(SelfTest, MessageType_SelfTest);
trezor_message_impl!(GetPublicKey, MessageType_GetPublicKey);
trezor_message_impl!(PublicKey, MessageType_PublicKey);
trezor_message_impl!(SignTx, MessageType_SignTx);
trezor_message_impl!(TxRequest, MessageType_TxRequest);
trezor_message_impl!(TxAck, MessageType_TxAck);
trezor_message_impl!(GetAddress, MessageType_GetAddress);
trezor_message_impl!(Address, MessageType_Address);
trezor_message_impl!(SignMessage, MessageType_SignMessage);
trezor_message_impl!(VerifyMessage, MessageType_VerifyMessage);
trezor_message_impl!(MessageSignature, MessageType_MessageSignature);
trezor_message_impl!(CipherKeyValue, MessageType_CipherKeyValue);
trezor_message_impl!(CipheredKeyValue, MessageType_CipheredKeyValue);
trezor_message_impl!(SignIdentity, MessageType_SignIdentity);
trezor_message_impl!(SignedIdentity, MessageType_SignedIdentity);
trezor_message_impl!(GetECDHSessionKey, MessageType_GetECDHSessionKey);
trezor_message_impl!(ECDHSessionKey, MessageType_ECDHSessionKey);
trezor_message_impl!(CosiCommit, MessageType_CosiCommit);
trezor_message_impl!(CosiCommitment, MessageType_CosiCommitment);
trezor_message_impl!(CosiSign, MessageType_CosiSign);
trezor_message_impl!(CosiSignature, MessageType_CosiSignature);
trezor_message_impl!(DebugLinkDecision, MessageType_DebugLinkDecision);
trezor_message_impl!(DebugLinkGetState, MessageType_DebugLinkGetState);
trezor_message_impl!(DebugLinkState, MessageType_DebugLinkState);
trezor_message_impl!(DebugLinkStop, MessageType_DebugLinkStop);
trezor_message_impl!(DebugLinkLog, MessageType_DebugLinkLog);
trezor_message_impl!(DebugLinkMemoryRead, MessageType_DebugLinkMemoryRead);
trezor_message_impl!(DebugLinkMemory, MessageType_DebugLinkMemory);
trezor_message_impl!(DebugLinkMemoryWrite, MessageType_DebugLinkMemoryWrite);
trezor_message_impl!(DebugLinkFlashErase, MessageType_DebugLinkFlashErase);
trezor_message_impl!(EthereumGetAddress, MessageType_EthereumGetAddress);
trezor_message_impl!(EthereumAddress, MessageType_EthereumAddress);
trezor_message_impl!(EthereumSignTx, MessageType_EthereumSignTx);
trezor_message_impl!(EthereumSignTxEIP1559, MessageType_EthereumSignTxEIP1559);
trezor_message_impl!(EthereumTxRequest, MessageType_EthereumTxRequest);
trezor_message_impl!(EthereumTxAck, MessageType_EthereumTxAck);
trezor_message_impl!(EthereumSignMessage, MessageType_EthereumSignMessage);
trezor_message_impl!(EthereumVerifyMessage, MessageType_EthereumVerifyMessage);
trezor_message_impl!(EthereumMessageSignature, MessageType_EthereumMessageSignature);
trezor_message_impl!(EthereumSignTypedData, MessageType_EthereumSignTypedData);
trezor_message_impl!(EthereumTypedDataStructRequest, MessageType_EthereumTypedDataStructRequest);
trezor_message_impl!(EthereumTypedDataStructAck, MessageType_EthereumTypedDataStructAck);
trezor_message_impl!(EthereumTypedDataValueRequest, MessageType_EthereumTypedDataValueRequest);
trezor_message_impl!(EthereumTypedDataValueAck, MessageType_EthereumTypedDataValueAck);
trezor_message_impl!(EthereumTypedDataSignature, MessageType_EthereumTypedDataSignature);
trezor_message_impl!(NEMGetAddress, MessageType_NEMGetAddress);
trezor_message_impl!(NEMAddress, MessageType_NEMAddress);
trezor_message_impl!(NEMSignTx, MessageType_NEMSignTx);
trezor_message_impl!(NEMSignedTx, MessageType_NEMSignedTx);
trezor_message_impl!(NEMDecryptMessage, MessageType_NEMDecryptMessage);
trezor_message_impl!(NEMDecryptedMessage, MessageType_NEMDecryptedMessage);
trezor_message_impl!(LiskGetAddress, MessageType_LiskGetAddress);
trezor_message_impl!(LiskAddress, MessageType_LiskAddress);
trezor_message_impl!(LiskSignTx, MessageType_LiskSignTx);
trezor_message_impl!(LiskSignedTx, MessageType_LiskSignedTx);
trezor_message_impl!(LiskSignMessage, MessageType_LiskSignMessage);
trezor_message_impl!(LiskMessageSignature, MessageType_LiskMessageSignature);
trezor_message_impl!(LiskVerifyMessage, MessageType_LiskVerifyMessage);
trezor_message_impl!(LiskGetPublicKey, MessageType_LiskGetPublicKey);
trezor_message_impl!(LiskPublicKey, MessageType_LiskPublicKey);
trezor_message_impl!(TezosGetAddress, MessageType_TezosGetAddress);
trezor_message_impl!(TezosAddress, MessageType_TezosAddress);
trezor_message_impl!(TezosSignTx, MessageType_TezosSignTx);
trezor_message_impl!(TezosSignedTx, MessageType_TezosSignedTx);
trezor_message_impl!(TezosGetPublicKey, MessageType_TezosGetPublicKey);
trezor_message_impl!(TezosPublicKey, MessageType_TezosPublicKey);
trezor_message_impl!(StellarSignTx, MessageType_StellarSignTx);
trezor_message_impl!(StellarTxOpRequest, MessageType_StellarTxOpRequest);
trezor_message_impl!(StellarGetAddress, MessageType_StellarGetAddress);
trezor_message_impl!(StellarAddress, MessageType_StellarAddress);
trezor_message_impl!(StellarCreateAccountOp, MessageType_StellarCreateAccountOp);
trezor_message_impl!(StellarPaymentOp, MessageType_StellarPaymentOp);
trezor_message_impl!(StellarPathPaymentOp, MessageType_StellarPathPaymentOp);
trezor_message_impl!(StellarManageOfferOp, MessageType_StellarManageOfferOp);
trezor_message_impl!(StellarCreatePassiveOfferOp, MessageType_StellarCreatePassiveOfferOp);
trezor_message_impl!(StellarSetOptionsOp, MessageType_StellarSetOptionsOp);
trezor_message_impl!(StellarChangeTrustOp, MessageType_StellarChangeTrustOp);
trezor_message_impl!(StellarAllowTrustOp, MessageType_StellarAllowTrustOp);
trezor_message_impl!(StellarAccountMergeOp, MessageType_StellarAccountMergeOp);
trezor_message_impl!(StellarManageDataOp, MessageType_StellarManageDataOp);
trezor_message_impl!(StellarBumpSequenceOp, MessageType_StellarBumpSequenceOp);
trezor_message_impl!(StellarSignedTx, MessageType_StellarSignedTx);
trezor_message_impl!(TronGetAddress, MessageType_TronGetAddress);
trezor_message_impl!(TronAddress, MessageType_TronAddress);
trezor_message_impl!(TronSignTx, MessageType_TronSignTx);
trezor_message_impl!(TronSignedTx, MessageType_TronSignedTx);
trezor_message_impl!(CardanoSignTx, MessageType_CardanoSignTx);
trezor_message_impl!(CardanoTxRequest, MessageType_CardanoTxRequest);
trezor_message_impl!(CardanoGetPublicKey, MessageType_CardanoGetPublicKey);
trezor_message_impl!(CardanoPublicKey, MessageType_CardanoPublicKey);
trezor_message_impl!(CardanoGetAddress, MessageType_CardanoGetAddress);
trezor_message_impl!(CardanoAddress, MessageType_CardanoAddress);
trezor_message_impl!(CardanoTxAck, MessageType_CardanoTxAck);
trezor_message_impl!(CardanoSignedTx, MessageType_CardanoSignedTx);
trezor_message_impl!(OntologyGetAddress, MessageType_OntologyGetAddress);
trezor_message_impl!(OntologyAddress, MessageType_OntologyAddress);
trezor_message_impl!(OntologyGetPublicKey, MessageType_OntologyGetPublicKey);
trezor_message_impl!(OntologyPublicKey, MessageType_OntologyPublicKey);
trezor_message_impl!(OntologySignTransfer, MessageType_OntologySignTransfer);
trezor_message_impl!(OntologySignedTransfer, MessageType_OntologySignedTransfer);
trezor_message_impl!(OntologySignWithdrawOng, MessageType_OntologySignWithdrawOng);
trezor_message_impl!(OntologySignedWithdrawOng, MessageType_OntologySignedWithdrawOng);
trezor_message_impl!(OntologySignOntIdRegister, MessageType_OntologySignOntIdRegister);
trezor_message_impl!(OntologySignedOntIdRegister, MessageType_OntologySignedOntIdRegister);
trezor_message_impl!(OntologySignOntIdAddAttributes, MessageType_OntologySignOntIdAddAttributes);
trezor_message_impl!(
	OntologySignedOntIdAddAttributes,
	MessageType_OntologySignedOntIdAddAttributes
);
trezor_message_impl!(RippleGetAddress, MessageType_RippleGetAddress);
trezor_message_impl!(RippleAddress, MessageType_RippleAddress);
trezor_message_impl!(RippleSignTx, MessageType_RippleSignTx);
trezor_message_impl!(RippleSignedTx, MessageType_RippleSignedTx);
trezor_message_impl!(MoneroTransactionInitRequest, MessageType_MoneroTransactionInitRequest);
trezor_message_impl!(MoneroTransactionInitAck, MessageType_MoneroTransactionInitAck);
trezor_message_impl!(
	MoneroTransactionSetInputRequest,
	MessageType_MoneroTransactionSetInputRequest
);
trezor_message_impl!(MoneroTransactionSetInputAck, MessageType_MoneroTransactionSetInputAck);
trezor_message_impl!(
	MoneroTransactionInputsPermutationRequest,
	MessageType_MoneroTransactionInputsPermutationRequest
);
trezor_message_impl!(
	MoneroTransactionInputsPermutationAck,
	MessageType_MoneroTransactionInputsPermutationAck
);
trezor_message_impl!(
	MoneroTransactionInputViniRequest,
	MessageType_MoneroTransactionInputViniRequest
);
trezor_message_impl!(MoneroTransactionInputViniAck, MessageType_MoneroTransactionInputViniAck);
trezor_message_impl!(
	MoneroTransactionAllInputsSetRequest,
	MessageType_MoneroTransactionAllInputsSetRequest
);
trezor_message_impl!(
	MoneroTransactionAllInputsSetAck,
	MessageType_MoneroTransactionAllInputsSetAck
);
trezor_message_impl!(
	MoneroTransactionSetOutputRequest,
	MessageType_MoneroTransactionSetOutputRequest
);
trezor_message_impl!(MoneroTransactionSetOutputAck, MessageType_MoneroTransactionSetOutputAck);
trezor_message_impl!(
	MoneroTransactionAllOutSetRequest,
	MessageType_MoneroTransactionAllOutSetRequest
);
trezor_message_impl!(MoneroTransactionAllOutSetAck, MessageType_MoneroTransactionAllOutSetAck);
trezor_message_impl!(
	MoneroTransactionMlsagDoneRequest,
	MessageType_MoneroTransactionMlsagDoneRequest
);
trezor_message_impl!(MoneroTransactionMlsagDoneAck, MessageType_MoneroTransactionMlsagDoneAck);
trezor_message_impl!(
	MoneroTransactionSignInputRequest,
	MessageType_MoneroTransactionSignInputRequest
);
trezor_message_impl!(MoneroTransactionSignInputAck, MessageType_MoneroTransactionSignInputAck);
trezor_message_impl!(MoneroTransactionFinalRequest, MessageType_MoneroTransactionFinalRequest);
trezor_message_impl!(MoneroTransactionFinalAck, MessageType_MoneroTransactionFinalAck);
trezor_message_impl!(MoneroKeyImageExportInitRequest, MessageType_MoneroKeyImageExportInitRequest);
trezor_message_impl!(MoneroKeyImageExportInitAck, MessageType_MoneroKeyImageExportInitAck);
trezor_message_impl!(MoneroKeyImageSyncStepRequest, MessageType_MoneroKeyImageSyncStepRequest);
trezor_message_impl!(MoneroKeyImageSyncStepAck, MessageType_MoneroKeyImageSyncStepAck);
trezor_message_impl!(MoneroKeyImageSyncFinalRequest, MessageType_MoneroKeyImageSyncFinalRequest);
trezor_message_impl!(MoneroKeyImageSyncFinalAck, MessageType_MoneroKeyImageSyncFinalAck);
trezor_message_impl!(MoneroGetAddress, MessageType_MoneroGetAddress);
trezor_message_impl!(MoneroAddress, MessageType_MoneroAddress);
trezor_message_impl!(MoneroGetWatchKey, MessageType_MoneroGetWatchKey);
trezor_message_impl!(MoneroWatchKey, MessageType_MoneroWatchKey);
trezor_message_impl!(DebugMoneroDiagRequest, MessageType_DebugMoneroDiagRequest);
trezor_message_impl!(DebugMoneroDiagAck, MessageType_DebugMoneroDiagAck);
