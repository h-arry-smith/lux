require_relative "lib/sacn"

class Output
end

class SACNOutput < Output
  def initialize(ip)
    @ip = ip
    @server = SACN::Server.new(@ip, "Lux")
  end

  def connect
    @server.connect()
  end

  def broadcast(universes)
    universes.each do |universe|
      packet = SACN::DataPacket.new(@server, universe.data, universe.number)
      @server.send(packet)
    end
  end
end
