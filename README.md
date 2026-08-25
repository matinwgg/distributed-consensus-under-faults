# Distributed Consensus Under Faults

## 📖 About

A research laboratory for studying distributed consensus when nodes may fail or behave maliciously, with emphasis on Byzantine faults, safety, liveness, network assumptions, and measurable fault tolerance.

## 🎯 Why It Exists

Consensus algorithms are defined by precise guarantees under explicit failure models. This project is intended to make those guarantees experimentally testable rather than treating consensus as a generic networking problem.

## ✨ Planned Features

- Byzantine-node simulation
- Message delay, loss, duplication, and reordering
- Consensus safety checks
- Liveness experiments
- Fault-threshold experiments
- Deterministic simulation traces
- Performance measurements

## 🛠 Tech Stack

- Rust (planned)
- Deterministic simulation tooling
- Property/invariant-based testing

## 🏗 Architecture

```text
Deterministic simulator
      ↓
Nodes + local state
      ↓
Adversarial network
      ↓
Consensus protocol
      ↓
State-machine transitions
      ↓
Safety/liveness checker
```

## 📁 Project Structure

Currently a scaffold. A mature implementation should separate protocol state, network simulation, adversary behavior, invariant checking, and experiment reporting.

## 📋 Prerequisites

No runnable implementation is currently documented.

## 🚀 Getting Started

```bash
git clone https://github.com/matinwgg/distributed-consensus-under-faults.git
cd distributed-consensus-under-faults
```

## 🧮 Mathematical Foundations

The project depends on graph connectivity, state machines, invariants, probability, combinatorics, quorum intersection, partial orders, and adversarial threshold reasoning.

## 🧪 Testing

Core tests should assert safety invariants under every generated fault trace and separately measure liveness under assumptions that permit progress.

## 🔐 Security Scope

Byzantine simulations are educational/research models. Claims must state the fault threshold, synchrony assumptions, cryptographic assumptions, and network model.

## 🚧 Future Work

- PBFT-style experiments
- Quorum intersection analysis
- Model checking
- Randomized consensus
- Fault injection benchmarks
- Visualization of counterexample traces

## 🤝 Contributing

Every protocol change should include a stated invariant and an executable adversarial scenario.

## 📄 License

See repository license information.

## 👨‍💻 Author

**Matin Odoom**
