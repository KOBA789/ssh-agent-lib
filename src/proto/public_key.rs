use super::error::ProtoError;
use super::key_type::{KeyType, KeyTypeEnum};
use super::private_key::*;
use serde::de::{Deserializer, Error};
use serde::ser::{SerializeTuple, Serializer};
use serde::{Deserialize, Serialize};

pub type MpInt = Vec<u8>;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct RsaPublicKey {
    pub e: MpInt,
    pub n: MpInt,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DssPublicKey {
    pub p: MpInt,
    pub q: MpInt,
    pub g: MpInt,
    pub y: MpInt,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct EcDsaPublicKey {
    pub identifier: String,
    pub q: MpInt,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SkEcDsaPublicKey {
    pub identifier: String,
    pub q: MpInt,
    pub application: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Ed25519PublicKey {
    pub enc_a: Vec<u8>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SkEd25519PublicKey {
    pub enc_a: Vec<u8>,
    pub application: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PublicKey {
    Dss(DssPublicKey),
    Ed25519(Ed25519PublicKey),
    SkEd25519(SkEd25519PublicKey),
    Rsa(RsaPublicKey),
    EcDsa(EcDsaPublicKey),
    SkEcDsa(SkEcDsaPublicKey),
}

impl KeyType for RsaPublicKey {
    const KEY_TYPE: &'static str = RsaPrivateKey::KEY_TYPE;
}

impl KeyType for DssPublicKey {
    const KEY_TYPE: &'static str = DssPrivateKey::KEY_TYPE;
}

impl KeyType for Ed25519PublicKey {
    const KEY_TYPE: &'static str = Ed25519PrivateKey::KEY_TYPE;
}

impl KeyType for EcDsaPublicKey {
    const KEY_TYPE: &'static str = EcDsaPrivateKey::KEY_TYPE;

    fn key_type(&self) -> String {
        format!("{}-{}", Self::KEY_TYPE, self.identifier)
    }
}

impl KeyType for SkEd25519PublicKey {
    const KEY_TYPE: &'static str = "sk-ssh-ed25519@openssh.com";
}

impl KeyType for SkEcDsaPublicKey {
    const KEY_TYPE: &'static str = "sk-ecdsa-sha2";

    fn key_type(&self) -> String {
        format!("{}-{}@openssh.com", Self::KEY_TYPE, self.identifier)
    }
}

impl From<PrivateKey> for PublicKey {
    fn from(key: PrivateKey) -> Self {
        match key {
            PrivateKey::Dss(key) => PublicKey::Dss(DssPublicKey::from(key)),
            PrivateKey::Ed25519(key) => PublicKey::Ed25519(Ed25519PublicKey::from(key)),
            PrivateKey::SkEd25519(key) => PublicKey::SkEd25519(SkEd25519PublicKey::from(key)),
            PrivateKey::Rsa(key) => PublicKey::Rsa(RsaPublicKey::from(key)),
            PrivateKey::EcDsa(key) => PublicKey::EcDsa(EcDsaPublicKey::from(key)),
            PrivateKey::SkEcDsa(key) => PublicKey::SkEcDsa(SkEcDsaPublicKey::from(key)),
        }
    }
}

impl From<RsaPrivateKey> for RsaPublicKey {
    fn from(key: RsaPrivateKey) -> Self {
        Self { e: key.e, n: key.n }
    }
}

impl From<DssPrivateKey> for DssPublicKey {
    fn from(key: DssPrivateKey) -> Self {
        Self {
            p: key.p,
            q: key.q,
            g: key.g,
            y: key.y,
        }
    }
}

impl From<EcDsaPrivateKey> for EcDsaPublicKey {
    fn from(key: EcDsaPrivateKey) -> Self {
        Self {
            identifier: key.identifier,
            q: key.q,
        }
    }
}

impl From<SkEcDsaPrivateKey> for SkEcDsaPublicKey {
    fn from(key: SkEcDsaPrivateKey) -> Self {
        Self {
            identifier: key.identifier,
            q: key.q,
            application: key.application,
        }
    }
}

impl From<Ed25519PrivateKey> for Ed25519PublicKey {
    fn from(key: Ed25519PrivateKey) -> Self {
        Self { enc_a: key.enc_a }
    }
}

impl From<SkEd25519PrivateKey> for SkEd25519PublicKey {
    fn from(key: SkEd25519PrivateKey) -> Self {
        Self {
            enc_a: key.enc_a,
            application: key.application,
        }
    }
}

impl From<&PrivateKey> for PublicKey {
    fn from(key: &PrivateKey) -> Self {
        Self::from(key.clone())
    }
}

impl From<&RsaPrivateKey> for RsaPublicKey {
    fn from(key: &RsaPrivateKey) -> Self {
        Self::from(key.clone())
    }
}

impl From<&DssPrivateKey> for DssPublicKey {
    fn from(key: &DssPrivateKey) -> Self {
        Self::from(key.clone())
    }
}

impl From<&EcDsaPrivateKey> for EcDsaPublicKey {
    fn from(key: &EcDsaPrivateKey) -> Self {
        Self::from(key.clone())
    }
}

impl From<&Ed25519PrivateKey> for Ed25519PublicKey {
    fn from(key: &Ed25519PrivateKey) -> Self {
        Self::from(key.clone())
    }
}

impl_key_type_enum_ser_de!(
    PublicKey,
    (PublicKey::Dss, DssPublicKey),
    (PublicKey::Rsa, RsaPublicKey),
    (PublicKey::EcDsa, EcDsaPublicKey),
    (PublicKey::SkEcDsa, SkEcDsaPublicKey),
    (PublicKey::Ed25519, Ed25519PublicKey),
    (PublicKey::SkEd25519, SkEd25519PublicKey)
);
