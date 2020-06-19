'use strict';

var path = require('path');
var should = require('chai').should();
var sinon = require('sinon');
var proxyquire = require('proxyquire');

describe('#defaultConfig', function() {
  var expectedExecPath = path.resolve(__dirname, process.env.HOME, './.bitgreen/data/bitgreend');

  it('will return expected configuration', function() {
    var config = JSON.stringify({
      network: 'livenet',
      port: 3001,
      services: [
        'bitgreend',
        'web'
      ],
      servicesConfig: {
        bitgreend: {
          connect: [{
            rpchost: '127.0.0.1',
            rpcport: 9998,
            rpcuser: 'dash',
            rpcpassword: 'local321',
            zmqpubrawtx: 'tcp://127.0.0.1:28332'
           }]
        }
      }
    }, null, 2);
    var defaultConfig = proxyquire('../../lib/scaffold/default-config', {
      fs: {
        existsSync: sinon.stub().returns(false),
        writeFileSync: function(path, data) {
          path.should.equal(process.env.HOME + '/.bitgreen/bitgreen-node.json');
          data.should.equal(config);
        },
        readFileSync: function() {
          return config;
        }
      },
      mkdirp: {
        sync: sinon.stub()
      }
    });
    var home = process.env.HOME;
    var info = defaultConfig();
    info.path.should.equal(home + '/.bitgreen');
    info.config.network.should.equal('livenet');
    info.config.port.should.equal(3001);
    info.config.services.should.deep.equal(['bitgreend', 'web']);
    var bitgreend = info.config.servicesConfig.bitgreend;
    should.exist(bitgreend);
  });
  it('will include additional services', function() {
    var config = JSON.stringify({
      network: 'livenet',
      port: 3001,
      services: [
        'bitgreend',
        'web',
        'insight-api',
        'insight-ui'
      ],
      servicesConfig: {
        bitgreend: {
          connect: [{
            rpchost: '127.0.0.1',
            rpcport: 9998,
            rpcuser: 'dash',
            rpcpassword: 'local321',
            zmqpubrawtx: 'tcp://127.0.0.1:28332'
          }]
        }
      }
    }, null, 2);
    var defaultConfig = proxyquire('../../lib/scaffold/default-config', {
      fs: {
        existsSync: sinon.stub().returns(false),
        writeFileSync: function(path, data) {
          path.should.equal(process.env.HOME + '/.bitgreen/bitgreen-node.json');
          data.should.equal(config);
        },
        readFileSync: function() {
          return config;
        }
      },
      mkdirp: {
        sync: sinon.stub()
      }
    });
    var home = process.env.HOME;
    var info = defaultConfig({
      additionalServices: ['insight-api', 'insight-ui']
    });
    info.path.should.equal(home + '/.bitgreen');
    info.config.network.should.equal('livenet');
    info.config.port.should.equal(3001);
    info.config.services.should.deep.equal([
      'bitgreend',
      'web',
      'insight-api',
      'insight-ui'
    ]);
    var bitgreend = info.config.servicesConfig.bitgreend;
    should.exist(bitgreend);
  });
});
