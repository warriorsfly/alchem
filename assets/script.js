
const loginUrl = "http://127.0.0.1:3000/api/user/login";
for (let index = 300000; index < 302000; index++) {
    var json = {"name":index.toString(),"password":"123456"};
    axios(
        {
            method: 'post',
            url: loginUrl,
            data:json
        }
    ).then(data=>{

        const socket = new WebSocket('ws://localhost:3000/ws');

        socket.addEventListener('open', function (event) {
            socket.send('Hello Server!');
        });

        socket.addEventListener('message', function (event) {
            console.log('Message from server ', event.data);
        });
    })
    

    
}

function 

