// Build a deterministic, source-only Horizon native bundle; Node.js, no packages.
const fs = require('node:fs');
const path = require('node:path');
const zlib = require('node:zlib');
const crypto = require('node:crypto');
const root = path.resolve(__dirname, '..');
const id = 'naruto_horizon';
const version = '0.1.0';
const compat = JSON.parse(fs.readFileSync(path.join(root, 'compatibility.json')));
const folder = path.join(root, 'mixins', id);
const mixin = fs.readFileSync(path.join(folder, 'horizon-mixin.toml'), 'utf8');
if (!mixin.includes(`horizon_revision = "${compat.horizon_revision}"`)) throw Error('Horizon pins differ');
const manifest = { schema: 1, id, version, game: compat.game, horizon: compat.horizon_revision, mixins: compat.horizon_mixins_revision };
const files = [['horizon-native.json', Buffer.from(JSON.stringify(manifest, null, 2)+'\n')]];
for (const name of fs.readdirSync(folder).sort()) {
  if (!/\.(rs|toml)$/.test(name)) throw Error('Unexpected bundle file: '+name);
  files.push([`mixins/${id}/${name}`, fs.readFileSync(path.join(folder, name))]);
}
const table = Array.from({length: 256}, (_, n) => {
  for (let i=0;i<8;i++) n = n&1 ? 0xedb88320^(n>>>1) : n>>>1;
  return n>>>0;
});
function crc32(bytes) {
  let n=0xffffffff;
  for(const byte of bytes) n=table[(n^byte)&255]^(n>>>8);
  return (n^0xffffffff)>>>0;
}
const entries=[], central=[];
let offset=0;
for (const [name, bytes] of files.sort(([a],[b])=>a.localeCompare(b,'en'))) {
  const filename=Buffer.from(name);
  const compressed=zlib.deflateRawSync(bytes, {level:9});
  const crc=crc32(bytes), date=(46<<9)|(1<<5)|1; // 2026-01-01
  const header=Buffer.alloc(30);
  header.writeUInt32LE(0x04034b50,0); header.writeUInt16LE(20,4);
  header.writeUInt16LE(8,8); header.writeUInt16LE(date,12);
  header.writeUInt32LE(crc,14); header.writeUInt32LE(compressed.length,18);
  header.writeUInt32LE(bytes.length,22); header.writeUInt16LE(filename.length,26);
  entries.push(header,filename,compressed);
  const dir=Buffer.alloc(46);
  dir.writeUInt32LE(0x02014b50,0); dir.writeUInt16LE(0x0314,4);
  dir.writeUInt16LE(20,6); dir.writeUInt16LE(8,10); dir.writeUInt16LE(date,14);
  dir.writeUInt32LE(crc,16); dir.writeUInt32LE(compressed.length,20);
  dir.writeUInt32LE(bytes.length,24); dir.writeUInt16LE(filename.length,28);
  dir.writeUInt32LE(0x81a40000,38); dir.writeUInt32LE(offset,42);
  central.push(dir,filename);
  offset+=header.length+filename.length+compressed.length;
}
const centralBytes=Buffer.concat(central), end=Buffer.alloc(22);
end.writeUInt32LE(0x06054b50,0); end.writeUInt16LE(files.length,8);
end.writeUInt16LE(files.length,10); end.writeUInt32LE(centralBytes.length,12);
end.writeUInt32LE(offset,16);
const archive=Buffer.concat([...entries,centralBytes,end]);
const output=process.argv[2] ? path.resolve(process.argv[2]) : path.join(root,'target',`${id}-${version}.hnmod`);
fs.mkdirSync(path.dirname(output),{recursive:true}); fs.writeFileSync(output,archive);
console.log(JSON.stringify({file:output,bytes:archive.length,sha256:crypto.createHash('sha256').update(archive).digest('hex')},null,2));
