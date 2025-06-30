import Donate from "../Donate";
import JoinWaitlist from "../JoinWaitlist";

export default function() {
    return (
        <section className="flex flex-row justify-around gap-20 flex-wrap py-4 mb-10">
            <JoinWaitlist/>
            <Donate/>
        </section>
    )
}