const ports = [3000, 3001];

for (const port of ports) {
    const listener = Deno.listen({ port, transport: "tcp" });
    console.log("Deno is listening on port: ", port);

    for await (const conn of listener) {
        const buf = new Uint8Array(1024);

        const n = await conn.read(buf);

        if (n == null) {
            console.log("Null, no stream");
            break;
        }

        const recievedText = new TextDecoder().decode(buf.subarray(0, n));
        console.log("Recieved text: ", recievedText);

        await conn.write(buf.subarray(0, n));

        conn.close();
    }
}
