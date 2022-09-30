require "socket"

module SACN
  class Server
    attr_reader :name, :priority
    
    def initialize(ip, name)
      @udp = UDPSocket.new
      @ip = ip
      @name = pack_name(name)
      @priority = 100
      @sequence_number = 1
    end

    def name=(new_name)
      @name = pack_name(new_name)
    end

    def connect(port = 5568)
      @udp.connect(@ip, port)
    end

    def send(packet)
      @udp.send(packet.data(@sequence_number), 0)
      @sequence_number += 1
    end

    private

    def pack_name(name)
      name[0...63].ljust(64, [0x00].pack("c")).unpack("U*").pack("C*")
    end
  end
end
