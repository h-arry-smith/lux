require "./lib/sacn"

server = SACN::Server.new("127.0.0.1", "test")
server.connect()

loop do
  dmx_data = []
  512.times { |_| dmx_data << rand(255) }

  2000.times do |u|
    packet = SACN::DataPacket.new(server, dmx_data, u + 1)
    server.send(packet)
  end

  sleep(1 / 30.0)
end
