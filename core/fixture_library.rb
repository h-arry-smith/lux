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
		
		private
		
		def generate_fixture(file)
			data = YAML.load_file(file)

			fixture_class = Class.new(Fixture) do
				name data["name"]
			end
			
			add_parameters(fixture_class, data["params"])
			
			if data["color"]
				fixture_class.color data["color"].to_sym
			end

			@fixtures[fixture_identifier(file)] = fixture_class
		end
	
		def add_parameters(fixture, params)
			params.each do |name, args|
				if group?(name)
					fixture.group group_sym(name) do
						add_parameters(fixture, args)
					end
				else
					add_parameter(fixture, name, args)
				end
			end
		end
	
		def group?(name)
			name.split(" ").first == "group"
		end
		
		def group_sym(name)
			groupless_name = name.split(" ")[1..]
			groupless_name.join("-").to_sym
		end
		
		def add_parameter(fixture, name, args)
			if args.nil?
				fixture.param name.to_sym
			else
				symbol_args = symbolize_args(args)
				fixture.param name.to_sym, **symbol_args
			end
		end
		
		def symbolize_args(args)
			args.transform_keys { |k| k.to_sym }
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
		
		def fixture_identifier(file)
			"#{file.parent.basename}/#{file.basename.to_s[..-5]}"
		end
	end
end
