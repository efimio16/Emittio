import { NextResponse } from "next/server";
import { MongoClient } from "mongodb";

const MONGODB_URI = process.env.MONGODB_URI!;
if (!MONGODB_URI) throw new Error('MONGODB_URI not specified.');

export async function POST(req: Request) {
    const { email } = await req.json();

    if (!email || !email.includes("@")) {
        return NextResponse.json({ error: "Invalid email" }, { status: 400 });
    }

    const client = new MongoClient(MONGODB_URI);
    await client.connect();
    const db = client.db("emittio");
    const subscribers = db.collection("subscribers");

    await subscribers.updateOne(
        { email },
        { $set: { email, createdAt: new Date() } },
        { upsert: true }
    );

    client.close();
    return NextResponse.json({ success: true });
}