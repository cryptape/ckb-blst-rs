//!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// DO NOT EDIT THIS FILE!!
// The file is generated from blst_minpk_test.go by generate.py
//!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
/*
 * Copyright Supranational LLC
 * Licensed under the Apache License, Version 2.0, see LICENSE for details.
 * SPDX-License-Identifier: Apache-2.0
 */

package blst

import (
	"crypto/rand"
	"fmt"
	"runtime"
	"testing"
)

// Min PK
type PublicKeyMinSig = P2Affine
type SignatureMinSig = P1Affine
type AggregateSignatureMinSig = P1Aggregate

// Names in this file must be unique to support min-sig so we can't use 'dst'
// here.
var dstMinSig = []byte("BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_NUL_")

func init() {
	// Use all cores when testing and benchmarking
	SetMaxProcs(runtime.GOMAXPROCS(0))
}

func TestInfinityMinSig(t *testing.T) {
	var infComp [48]byte
	infComp[0] |= 0xc0
	new(PublicKeyMinSig).Uncompress(infComp[:])
}

func TestSerdesMinSig(t *testing.T) {
	var ikm = [...]byte{
		0x93, 0xad, 0x7e, 0x65, 0xde, 0xad, 0x05, 0x2a,
		0x08, 0x3a, 0x91, 0x0c, 0x8b, 0x72, 0x85, 0x91,
		0x46, 0x4c, 0xca, 0x56, 0x60, 0x5b, 0xb0, 0x56,
		0xed, 0xfe, 0x2b, 0x60, 0xa6, 0x3c, 0x48, 0x99}

	sk := KeyGen(ikm[:])

	// Serialize/deserialize sk
	sk2 := new(SecretKey).Deserialize(sk.Serialize())
	if !sk.Equals(sk2) {
		t.Errorf("sk2 != sk")
	}

	// Negative test equals
	sk.l[0] = sk.l[0] + 1
	if sk.Equals(sk2) {
		t.Errorf("sk2 == sk")
	}

	// pk
	pk := new(PublicKeyMinSig).From(sk)

	// Compress/decompress sk
	pk2 := new(PublicKeyMinSig).Uncompress(pk.Compress())
	if !pk.Equals(pk2) {
		t.Errorf("pk2 != pk")
	}

	// Serialize/deserialize sk
	pk3 := new(PublicKeyMinSig).Deserialize(pk.Serialize())
	if !pk.Equals(pk3) {
		t.Errorf("pk3 != pk")
	}

	// Negative test equals
	// pk.x.l[0] = pk.x.l[0] + 1
	// if pk.Equals(pk2) {
	// 	t.Errorf("pk2 == pk")
	// }
}

func TestSignVerifyMinSig(t *testing.T) {
	var ikm = [...]byte{
		0x93, 0xad, 0x7e, 0x65, 0xde, 0xad, 0x05, 0x2a,
		0x08, 0x3a, 0x91, 0x0c, 0x8b, 0x72, 0x85, 0x91,
		0x46, 0x4c, 0xca, 0x56, 0x60, 0x5b, 0xb0, 0x56,
		0xed, 0xfe, 0x2b, 0x60, 0xa6, 0x3c, 0x48, 0x99}

	sk0 := KeyGen(ikm[:])
	ikm[0] = ikm[0] + 1
	sk1 := KeyGen(ikm[:])

	// pk
	pk0 := new(PublicKeyMinSig).From(sk0)
	pk1 := new(PublicKeyMinSig).From(sk1)

	// Sign
	msg0 := []byte("hello foo")
	msg2 := []byte("hello bar!")
	sig0 := new(SignatureMinSig).Sign(sk0, msg0, dstMinSig)
	sig2 := new(SignatureMinSig).Sign(sk1, msg2, dstMinSig)

	// Verify
	if !sig0.Verify(pk0, msg0, dstMinSig) {
		t.Errorf("verify sig0")
	}
	if !sig2.Verify(pk1, msg2, dstMinSig) {
		t.Errorf("verify sig2")
	}
	if !new(SignatureMinSig).VerifyCompressed(sig2.Compress(), pk1.Compress(),
		msg2, dstMinSig) {
		t.Errorf("verify sig2")
	}
	// Batch verify
	if !sig0.AggregateVerify([]*PublicKeyMinSig{pk0}, []Message{msg0}, dstMinSig) {
		t.Errorf("aggregate verify sig0")
	}
	// Verify compressed inputs
	if !new(SignatureMinSig).AggregateVerifyCompressed(sig0.Compress(),
		[][]byte{pk0.Compress()}, []Message{msg0}, dstMinSig) {
		t.Errorf("aggregate verify sig0 compressed")
	}

	// Verify serialized inputs
	if !new(SignatureMinSig).AggregateVerifyCompressed(sig0.Serialize(),
		[][]byte{pk0.Serialize()}, []Message{msg0}, dstMinSig) {
		t.Errorf("aggregate verify sig0 serialized")
	}

	// Compressed with empty pk
	var emptyPk []byte
	if new(SignatureMinSig).VerifyCompressed(sig0.Compress(), emptyPk, msg0, dstMinSig) {
		t.Errorf("verify sig compressed inputs")
	}
	// Wrong message
	if sig0.Verify(pk0, msg2, dstMinSig) {
		t.Errorf("Expected Verify to return false")
	}
	// Wrong key
	if sig0.Verify(pk1, msg0, dstMinSig) {
		t.Errorf("Expected Verify to return false")
	}
	// Wrong sig
	if sig2.Verify(pk0, msg0, dstMinSig) {
		t.Errorf("Expected Verify to return false")
	}
}

func TestSignVerifyAugMinSig(t *testing.T) {
	sk := genRandomKeyMinSig()
	pk := new(PublicKeyMinSig).From(sk)
	msg := []byte("hello foo")
	aug := []byte("augmentation")
	sig := new(SignatureMinSig).Sign(sk, msg, dstMinSig, aug)
	if !sig.Verify(pk, msg, dstMinSig, aug) {
		t.Errorf("verify sig")
	}
	aug1 := []byte("augmentation2")
	if sig.Verify(pk, msg, dstMinSig, aug1) {
		t.Errorf("verify sig, wrong augmentation")
	}
	if sig.Verify(pk, msg, dstMinSig) {
		t.Errorf("verify sig, no augmentation")
	}
	// TODO: augmentation with aggregate verify
}

func TestSignVerifyEncodeMinSig(t *testing.T) {
	sk := genRandomKeyMinSig()
	pk := new(PublicKeyMinSig).From(sk)
	msg := []byte("hello foo")
	sig := new(SignatureMinSig).Sign(sk, msg, dstMinSig, false)
	if !sig.Verify(pk, msg, dstMinSig, false) {
		t.Errorf("verify sig")
	}
	if sig.Verify(pk, msg, dstMinSig) {
		t.Errorf("verify sig expected fail, wrong hashing engine")
	}
	if sig.Verify(pk, msg, dstMinSig, 0) {
		t.Errorf("verify sig expected fail, illegal argument")
	}
}

func TestSignVerifyAggregateMinSig(t *testing.T) {
	for size := 1; size < 20; size++ {
		sks, msgs, _, pubks, _ := generateBatchTestDataUncompressedMinSig(size)

		// All signers sign the same message
		sigs := make([]*SignatureMinSig, 0)
		for i := 0; i < size; i++ {
			sigs = append(sigs, new(SignatureMinSig).Sign(sks[i], msgs[0],
				dstMinSig))
		}
		agSig := new(AggregateSignatureMinSig).Aggregate(sigs).ToAffine()

		if !agSig.FastAggregateVerify(pubks, msgs[0], dstMinSig) {
			t.Errorf("failed to verify size %d", size)
		}

		// Test compressed/serialized signature aggregation
		compSigs := make([][]byte, size)
		for i := 0; i < size; i++ {
			if (i % 2) == 0 {
				compSigs[i] = sigs[i].Compress()
			} else {
				compSigs[i] = sigs[i].Serialize()
			}
		}
		agSig = new(AggregateSignatureMinSig).AggregateCompressed(compSigs).
			ToAffine()
		if !agSig.FastAggregateVerify(pubks, msgs[0], dstMinSig) {
			t.Errorf("failed to verify size %d", size)
		}

	}
}

func BenchmarkCoreSignMinSig(b *testing.B) {
	var ikm = [...]byte{
		0x93, 0xad, 0x7e, 0x65, 0xde, 0xad, 0x05, 0x2a,
		0x08, 0x3a, 0x91, 0x0c, 0x8b, 0x72, 0x85, 0x91,
		0x46, 0x4c, 0xca, 0x56, 0x60, 0x5b, 0xb0, 0x56,
		0xed, 0xfe, 0x2b, 0x60, 0xa6, 0x3c, 0x48, 0x99}

	sk := KeyGen(ikm[:])
	msg := []byte("hello foo")
	for i := 0; i < b.N; i++ {
		new(SignatureMinSig).Sign(sk, msg, dstMinSig)
	}
}

func BenchmarkCoreVerifyMinSig(b *testing.B) {
	var ikm = [...]byte{
		0x93, 0xad, 0x7e, 0x65, 0xde, 0xad, 0x05, 0x2a,
		0x08, 0x3a, 0x91, 0x0c, 0x8b, 0x72, 0x85, 0x91,
		0x46, 0x4c, 0xca, 0x56, 0x60, 0x5b, 0xb0, 0x56,
		0xed, 0xfe, 0x2b, 0x60, 0xa6, 0x3c, 0x48, 0x99}

	sk := KeyGen(ikm[:])
	pk := new(PublicKeyMinSig).From(sk)
	msg := []byte("hello foo")
	sig := new(SignatureMinSig).Sign(sk, msg, dstMinSig)

	// Verify
	for i := 0; i < b.N; i++ {
		if !sig.Verify(pk, msg, dstMinSig) {
			b.Fatal("verify sig")
		}
	}
}

func BenchmarkCoreVerifyAggregateMinSig(b *testing.B) {
	run := func(size int) func(b *testing.B) {
		return func(b *testing.B) {
			msgs, _, pubks, agsig := generateBatchTestDataMinSig(size)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				if !new(SignatureMinSig).AggregateVerifyCompressed(agsig, pubks,
					msgs, dstMinSig) {
					b.Fatal("failed to verify")
				}
			}
		}
	}

	b.Run("1", run(1))
	b.Run("10", run(10))
	b.Run("50", run(50))
	b.Run("100", run(100))
	b.Run("300", run(300))
	b.Run("1000", run(1000))
	b.Run("4000", run(4000))
}

func BenchmarkVerifyAggregateUncompressedMinSig(b *testing.B) {
	run := func(size int) func(b *testing.B) {
		return func(b *testing.B) {
			_, msgs, _, pubks, agsig :=
				generateBatchTestDataUncompressedMinSig(size)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				if !agsig.AggregateVerify(pubks, msgs, dstMinSig) {
					b.Fatal("failed to verify")
				}
			}
		}
	}

	b.Run("1", run(1))
	b.Run("10", run(10))
	b.Run("50", run(50))
	b.Run("100", run(100))
	b.Run("300", run(300))
	b.Run("1000", run(1000))
	b.Run("4000", run(4000))
}

func BenchmarkCoreAggregateMinSig(b *testing.B) {
	run := func(size int) func(b *testing.B) {
		return func(b *testing.B) {
			_, sigs, _, _ := generateBatchTestDataMinSig(size)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				var agg AggregateSignatureMinSig
				agg.AggregateCompressed(sigs)
			}
		}
	}

	b.Run("1", run(1))
	b.Run("10", run(10))
	b.Run("50", run(50))
	b.Run("100", run(100))
	b.Run("300", run(300))
	b.Run("1000", run(1000))
	b.Run("4000", run(4000))
}

func genRandomKeyMinSig() *SecretKey {
	// Generate 32 bytes of randomness
	var ikm [32]byte
	_, err := rand.Read(ikm[:])

	if err != nil {
		return nil
	}
	return KeyGen(ikm[:])
}

func generateBatchTestDataMinSig(size int) (msgs []Message,
	sigs [][]byte, pubks [][]byte, agsig []byte) {
	for i := 0; i < size; i++ {
		msg := Message(fmt.Sprintf("blst is a blast!! %d", i))
		msgs = append(msgs, msg)
		priv := genRandomKeyMinSig()
		sigs = append(sigs, new(SignatureMinSig).Sign(priv, msg, dstMinSig).
			Compress())
		pubks = append(pubks, new(PublicKeyMinSig).From(priv).Compress())
	}
	agsig = new(AggregateSignatureMinSig).AggregateCompressed(sigs).ToAffine().
		Compress()
	return
}

func generateBatchTestDataUncompressedMinSig(size int) (sks []*SecretKey,
	msgs []Message, sigs []*SignatureMinSig, pubks []*PublicKeyMinSig,
	agsig *SignatureMinSig) {
	for i := 0; i < size; i++ {
		msg := Message(fmt.Sprintf("blst is a blast!! %d", i))
		msgs = append(msgs, msg)
		priv := genRandomKeyMinSig()
		sks = append(sks, priv)
		sigs = append(sigs, new(SignatureMinSig).Sign(priv, msg, dstMinSig))
		pubks = append(pubks, new(PublicKeyMinSig).From(priv))
	}
	agsig = new(AggregateSignatureMinSig).Aggregate(sigs).ToAffine()
	return
}
