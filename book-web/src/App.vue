<template>
  <div class="app-container">
    <h1 class="title">📚 BookStore</h1>

    <div v-if="!connected" class="button-container">
      <input v-model="accountIndex">
      <button @click="connect">Connect Wallet</button>
    </div>

    <div v-else class="connected-container">
      <p class="account-info">Connected as: {{ account.meta.name }} ({{ account.address }})</p>

      <h2 class="sub-title">Your Books</h2>
      <ul class="candidate-list">
        <li v-for="book in ownedBooks" :key="book.id" class="candidate-item">
          <span class="candidate-name">{{ book.title }} by {{ book.author }}</span>
          <div>
            <button v-if="!book.forSale" @click="sellBook(book.id)">Put for Sale</button>
            <button @click="viewContent(book.id)">Read</button>
          </div>
        </li>
      </ul>

      <h2 class="sub-title">Books for Sale</h2>
      <ul class="candidate-list">
        <li v-for="book in booksForSale" :key="book.id" class="candidate-item">
          <span class="candidate-name">{{ book.title }} by {{ book.author }}</span> - {{ book.price }}
          <button @click="buyBook(book.id)">Buy</button>
        </li>
      </ul>

      <h2 class="sub-title">Add New Book</h2>
      <form @submit.prevent="addBook">
        <input v-model="newBook.title" placeholder="Title" required/>
        <input v-model="newBook.author" placeholder="Author" required/>
        <input v-model="newBook.content" placeholder="Content" required/>
        <input v-model.number="newBook.price" type="number" placeholder="Price" required/>
        <button type="submit">Add Book</button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">import {ref} from 'vue';
import {web3Accounts, web3Enable, web3FromSource} from '@polkadot/extension-dapp';
import {ApiPromise, WsProvider} from '@polkadot/api';
import {ContractPromise} from '@polkadot/api-contract';
import metadata from '@/contract/book_contract.json'; // Replace with your new metadata!

const CONTRACT_ADDRESS = import.meta.env.VITE_APP_CONTRACT_ADDRESS;
const WS_PROVIDER = import.meta.env.VITE_APP_WS_PROVIDER;

const connected = ref(false);
const account = ref<any>(null);
const accountIndex = ref(0)
const contract = ref<any>(null);
const ownedBooks = ref<any[]>([]);
const booksForSale = ref<any[]>([]);

const newBook = ref({
  title: '',
  author: '',
  content: '',
  price: 0,
});

async function connect() {
  await web3Enable('BookStore');
  const allAccounts = await web3Accounts();
  account.value = allAccounts[accountIndex.value];

  const wsProvider = new WsProvider(WS_PROVIDER);
  const api = await ApiPromise.create({provider: wsProvider});
  contract.value = new ContractPromise(api, metadata, CONTRACT_ADDRESS);

  const injector = await web3FromSource(account.value.meta.source);
  api.setSigner(injector.signer);

  connected.value = true;
  await fetchBooks();
}

function getGas() {
  return contract.value.api.registry.createType('WeightV2', {
    refTime: Number.MAX_SAFE_INTEGER,
    proofSize: Number.MAX_SAFE_INTEGER,
  });
}

async function fetchBooks() {
  const {output: owned} = await contract.value.query.ownedBooks(account.value.address, {
    gasLimit: getGas(),
  });
  ownedBooks.value = owned?.toJSON().ok ?? [];

  const {output: forSale} = await contract.value.query.booksForSale(account.value.address, {
    gasLimit: getGas(),
  });
  booksForSale.value = forSale?.toJSON().ok ?? [];
}

async function addBook() {
  const gas = await contract.value.query.addBook(account.value.address, {
    gasLimit: getGas()
  }, newBook.value.title, newBook.value.author, newBook.value.content, newBook.value.price);

  const tx = await contract.value.tx.addBook({gasLimit: gas.gasRequired}, newBook.value.title, newBook.value.author, newBook.value.content, newBook.value.price);

  await tx.signAndSend(account.value.address, ({status}) => {
    if (status.isInBlock || status.isFinalized) {
      fetchBooks();
    }
  });
}

async function buyBook(id: number) {
  const gas = await contract.value.query.buy(account.value.address, {gasLimit: getGas()}, id);

  const tx = await contract.value.tx.buy({gasLimit: gas.gasRequired}, id);

  await tx.signAndSend(account.value.address, ({status}) => {
    if (status.isInBlock || status.isFinalized) {
      fetchBooks();
    }
  });
}

async function sellBook(id: number) {
  const gas = await contract.value.query.sell(account.value.address, {gasLimit: getGas()}, id);

  const tx = await contract.value.tx.sell({gasLimit: gas.gasRequired}, id);

  await tx.signAndSend(account.value.address, ({status}) => {
    if (status.isInBlock || status.isFinalized) {
      fetchBooks();
    }
  });
}

async function viewContent(id: number) {
  const {output} = await contract.value.query.getContent(account.value.address, {
    gasLimit: getGas(),
  }, id);
  alert(`Content:\n\n${JSON.stringify(output?.toJSON().ok)}`);
}

</script>

<style><
style scoped >
.app-container {
  max-width: 800px;
  margin: 2rem auto;
  padding: 1.5rem;
  font-family: 'Segoe UI', sans-serif;
  background-color: #fdfdfd;
  border: 1px solid #ddd;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.title {
  text-align: center;
  font-size: 2rem;
  margin-bottom: 1.5rem;
}

.sub-title {
  margin-top: 2rem;
  font-size: 1.25rem;
  border-bottom: 1px solid #ccc;
  padding-bottom: 0.5rem;
}

.account-info {
  margin-bottom: 1rem;
  font-size: 0.95rem;
  color: #333;
  background: #f1f1f1;
  padding: 0.75rem;
  border-radius: 8px;
}

.button-container {
  text-align: center;
  margin-top: 2rem;
}

button {
  background-color: #007acc;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  margin: 0.25rem;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.2s ease-in-out;
}

button:hover {
  background-color: #005fa3;
}

input {
  margin: 0.5rem 0;
  padding: 0.5rem;
  width: 100%;
  box-sizing: border-box;
  border: 1px solid #ccc;
  border-radius: 6px;
}

form {
  margin-top: 1rem;
  display: flex;
  flex-direction: column;
}

.candidate-list {
  list-style: none;
  padding: 0;
  margin: 0.5rem 0;
}

.candidate-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: #f9f9f9;
  padding: 0.75rem;
  margin-bottom: 0.5rem;
  border-radius: 6px;
  border: 1px solid #eee;
}

.candidate-name {
  font-weight: bold;
  font-size: 1rem;
}

@media (max-width: 600px) {
  .candidate-item {
    flex-direction: column;
    align-items: flex-start;
  }

  button {
    width: 100%;
    margin: 0.25rem 0;
  }
}

</style>
