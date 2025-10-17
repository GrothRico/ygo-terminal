// intended to be pasted into a browser console, doesn't seem to work in Firefox

var dc;
(async () => {
  const role = prompt(
    "Enter role: 'A' (offerer) or 'B' (answerer)",
  ).toUpperCase();
  const pc = new RTCPeerConnection({
    iceServers: [{ urls: "stun:stun.l.google.com:19302" }],
  });

  if (role === "B") {
    pc.ondatachannel = (e) => {
      dc = e.channel;
      dc.onopen = () => {
        console.log("Connected! Use dc.send(msg) to send messages.");
      };
      dc.onmessage = (ev) => console.log("A says:", ev.data);
    };
  }

  if (role === "A") {
    dc = pc.createDataChannel("chat");
    dc.onopen = () => {
      console.log("Connected! Use dc.send(msg) to send messages.");
    };
    dc.onmessage = (e) => console.log("B says:", e.data);

    const offer = await pc.createOffer();
    await pc.setLocalDescription(offer);

    while (pc.iceGatheringState !== "complete")
      await new Promise((r) => setTimeout(r, 50));
    console.log(JSON.stringify(pc.localDescription));

    const answerStr = prompt("Paste answer from B:");
    const answer = JSON.parse(answerStr);
    await pc.setRemoteDescription(answer);
  } else {
    const offerStr = prompt("Paste offer from A:");
    const offer = JSON.parse(offerStr);
    await pc.setRemoteDescription(offer);

    const answer = await pc.createAnswer();
    await pc.setLocalDescription(answer);

    while (pc.iceGatheringState !== "complete")
      await new Promise((r) => setTimeout(r, 50));
    console.log(JSON.stringify(pc.localDescription));
  }
})();
