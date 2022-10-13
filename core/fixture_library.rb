require "pathname"
require "yaml"
require_relative "fixture"

module Core
	class FixtureLibrary
		def initialize(path)
			@path = Pathname.new(path)
			@fixtures = {}
			generate_directory(@path)
		end
		
		def [](identifier)
			@fixtures[identifier]
		end
		
		def generate_fixture(file)
			data = YAML.load_file(file)
			fixture_class = Class.new(Fixture) do
				name data["name"]
				data["params"].each do |name, args|
					if args.nil?
						param name.to_sym
					else
						param name.to_sym, **args
					end
				end
			end
			
			@fixtures[fixture_identifier(file)] = fixture_class
		end
		
		def generate_directory(directory_path)
			directory_path.children.each do |file|
				if file.directory?
					generate_directory(file)
				else
					generate_fixture(file)
				end
			end
		end
		
		private
		
		def fixture_identifier(file)
			"#{file.parent.basename}/#{file.basename.to_s[..-5]}"
		end
	end
end
