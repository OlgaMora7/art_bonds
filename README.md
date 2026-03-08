# 🎨 Art Bonds (Bonos de Arte Digital) - Solana Smart Contract

Este proyecto es un contrato inteligente (Backend) desarrollado en Rust usando el framework Anchor para la red de Solana. 

## 📌 Descripción del Proyecto
"Art Bonds" fusiona el arte digital con las finanzas descentralizadas (DeFi). Permite a los usuarios emitir un "Bono de Arte" depositando un capital inicial (SOL). A través del contrato, el bono genera un rendimiento simulado basado en el tiempo que el capital permanece bloqueado, funcionando como un instrumento financiero nativo de la blockchain.

## ⚙️ Arquitectura Técnica (Requisitos de Certificación)

Este proyecto implementa un **CRUD** completo y utiliza **PDA (Program Derived Addresses)** para gestionar el estado y los fondos de forma segura y descentralizada.

### 1. Program Derived Address (PDA)
La cuenta que actúa como "Bóveda" y "Libro Mayor" del bono es una PDA. 
- **Semilla:** Se deriva usando el string `b"bond"` y la llave pública del usuario `user.key()`.
- **Seguridad:** El contrato es la única entidad capaz de firmar transacciones de retiro o actualización sobre esta cuenta. La PDA garantiza que no exista una llave privada que pueda comprometer los fondos.

### 2. Implementación del CRUD
El flujo de datos interactúa con la estructura `BondState` de la siguiente manera:

* **Create (`issue_bond`):** Inicializa la cuenta PDA del bono, registra el capital inicial (`principal`), asocia al propietario (`owner`) y estampa el *timestamp* de emisión. Transfiere los fondos de la wallet del usuario a la PDA.
* **Read (`BondState`):** La estructura pública permite a cualquier cliente web3 consultar el estado del bono (propietario, capital aportado y última fecha de cobro) sin costo de gas.
* **Update (`update_yield`):** Simula el devengo de intereses. Calcula el tiempo transcurrido desde el último reclamo y actualiza la variable `last_claim_date` mutando el estado de la PDA.
* **Delete (`redeem_bond`):** Liquida el bono financiero. Utiliza el atributo `close = user` para destruir la cuenta PDA, devolviendo automáticamente el capital inicial y la exención de renta a la wallet del usuario.

## 🚀 Cómo ejecutar localmente

### Prerrequisitos
- Rust
- Solana CLI
- Node.js & Yarn
- Anchor CLI

### Pruebas
1. Clona este repositorio y corre `yarn install`.
2. Ejecuta el simulador con:
   ```bash
   anchor test