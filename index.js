const img = require('.').printscreen();
const fs = require('fs');

const main=()=>{
    fs.readFile('screenshot.png',(err,data)=>{
      console.log(data);
   });
   
   fs.writeFile('image.png',Buffer.from(img),(err)=>{
       console.log(err)
   })
}

main();