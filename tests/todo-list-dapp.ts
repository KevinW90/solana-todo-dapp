import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TodoListDapp } from "../target/types/todo_list_dapp";
import { assert } from "chai";

describe("todo-list-dapp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TodoListDapp as Program<TodoListDapp>;
  const author = program.provider as anchor.AnchorProvider;

  it("can create a task", async () => {
    const task = anchor.web3.Keypair.generate();
    console.log("Task account: ", task.publicKey.toBase58());
    const tx = await program.methods
      .addTask("You are awesome!")
      .accounts({
        task: task.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([task])
      .rpc();

    console.log("Transaction signature", tx);

    const taskAccount = await program.account.task.fetch(task.publicKey);
    console.log("Your task: ", taskAccount);

    assert.equal(
      taskAccount.author.toBase58(),
      author.wallet.publicKey.toBase58()
    );
    assert.equal(taskAccount.text, "You are awesome!");
    assert.equal(taskAccount.isDone, false);
    assert.ok(taskAccount.createdAt);
    assert.ok(taskAccount.updatedAt);
  });

  it("can mark a task as done", async () => {
    const task = anchor.web3.Keypair.generate();
    console.log("Task account: ", task.publicKey.toBase58());
    await program.methods
      .addTask("Adding 1 task")
      .accounts({
        task: task.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([task])
      .rpc();

    await program.methods
      .updateTask(true)
      .accounts({
        task: task.publicKey,
        author: author.publicKey,
      })
      .rpc();

    const taskAccount = await program.account.task.fetch(task.publicKey);
    // const updateTx = await program.methods
    //   .updateTask(true)
    //   .accounts({
    //     task: task.publicKey,
    //     author: authorKeyPair.publicKey,
    //   })
    //   .signers([authorKeyPair])
    //   .rpc();
    // console.log("update tx: ", updateTx);

    assert.equal(taskAccount.isDone, true);
  });
});
