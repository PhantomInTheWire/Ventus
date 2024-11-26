import SwiftUI
import UniformTypeIdentifiers
import MobileCoreServices

struct SyncView: View {
    @State private var progress: Float = 0.0
    @State private var rotation: Double = 0.0
    @State private var showingDoneView = false
    @State private var isShowingFilePicker = false
    @State private var selectedURL: URL?
    @State private var particles: [(offset: CGSize, scale: CGFloat, angle: Double)] = (0..<30).map { _ in
        (CGSize(width: .random(in: -150...150), height: .random(in: -150...150)),
         CGFloat.random(in: 0.2...0.7),
         .random(in: 0...360))
    }
    let ipAddress: String
    let localDir: String
    
    private let gradientColors = [
        Color(#colorLiteral(red: 0.1568627451, green: 0.1568627451, blue: 0.4, alpha: 1)),
        Color(#colorLiteral(red: 0.07843137255, green: 0.07843137255, blue: 0.2392156863, alpha: 1))
    ]
    
    var body: some View {
        GeometryReader { geometry in
            ZStack {
                // Background gradient
                LinearGradient(colors: gradientColors, startPoint: .top, endPoint: .bottom)
                    .ignoresSafeArea()
                
                // Particle system
                ForEach(0..<particles.count, id: \.self) { index in
                    Circle()
                        .fill(.white.opacity(0.5))
                        .frame(width: 4, height: 4)
                        .scaleEffect(particles[index].scale)
                        .offset(particles[index].offset)
                        .rotationEffect(.degrees(particles[index].angle))
                        .blur(radius: 1)
                        .onAppear {
                            let duration = Double.random(in: 2...4)
                            let distance = sqrt(pow(particles[index].offset.width, 2) +
                                                pow(particles[index].offset.height, 2))
                            let endOffset = CGSize(width: particles[index].offset.width * 0.2,
                                                   height: particles[index].offset.height * 0.2)
                            withAnimation(
                                Animation.easeInOut(duration: duration)
                                    .repeatForever(autoreverses: false)
                            ) {
                                particles[index].offset = endOffset
                            }
                            withAnimation(
                                Animation.linear(duration: duration)
                                    .repeatForever(autoreverses: false)
                            ) {
                                particles[index].angle += distance > 0 ? 360 : 0
                            }
                        }
                }
                
                VStack(spacing: 30) {
                    Spacer()
                    
                    if selectedURL == nil {
                        Text("Select Files to Sync")
                            .font(.system(size: 28, weight: .bold))
                            .foregroundStyle(
                                LinearGradient(
                                    colors: [.blue.opacity(0.8), .purple.opacity(0.8)],
                                    startPoint: .leading,
                                    endPoint: .trailing
                                )
                            )
                            .shadow(color: .blue.opacity(0.3), radius: 10)
                        
                        Button(action: {
                            isShowingFilePicker = true
                        }) {
                            Text("Choose Directory")
                                .font(.headline)
                                .foregroundColor(.white)
                                .padding()
                                .background(
                                    LinearGradient(
                                        colors: [.blue, .purple],
                                        startPoint: .leading,
                                        endPoint: .trailing
                                    )
                                )
                                .cornerRadius(10)
                        }
                    } else {
                        Text("Let the Magic Begin...")
                            .font(.system(size: 28, weight: .bold))
                            .foregroundStyle(
                                LinearGradient(
                                    colors: [.blue.opacity(0.8), .purple.opacity(0.8)],
                                    startPoint: .leading,
                                    endPoint: .trailing
                                )
                            )
                            .shadow(color: .blue.opacity(0.3), radius: 10)
                        
                        // Main progress circle
                        ZStack {
                            Circle()
                                .fill(
                                    RadialGradient(
                                        colors: [.blue.opacity(0.2), .clear],
                                        center: .center,
                                        startRadius: 80,
                                        endRadius: 150
                                    )
                                )
                                .frame(width: geometry.size.width * 0.7)
                                .blur(radius: 20)
                            
                            ForEach(0..<3) { index in
                                Circle()
                                    .stroke(
                                        LinearGradient(
                                            colors: [.blue.opacity(0.2), .purple.opacity(0.1)],
                                            startPoint: .top,
                                            endPoint: .bottom
                                        ),
                                        lineWidth: 1
                                    )
                                    .frame(width: geometry.size.width * 0.5)
                                    .scaleEffect(1 + CGFloat(index) * 0.2)
                                    .opacity(1 - Double(progress))
                                    .animation(
                                        .easeInOut(duration: 1.5)
                                        .repeatForever(autoreverses: false)
                                        .delay(Double(index) * 0.5),
                                        value: progress
                                    )
                            }
                            
                            // Progress components remain the same...
                            Circle()
                                .stroke(.white.opacity(0.1), lineWidth: 12)
                                .frame(width: geometry.size.width * 0.5)
                            
                            Circle()
                                .trim(from: 0, to: CGFloat(progress))
                                .stroke(
                                    LinearGradient(
                                        colors: [.blue, .purple],
                                        startPoint: .leading,
                                        endPoint: .trailing
                                    ),
                                    style: StrokeStyle(lineWidth: 12, lineCap: .round)
                                )
                                .frame(width: geometry.size.width * 0.5)
                                .rotationEffect(.degrees(-90))
                            
                            Circle()
                                .trim(from: 0.4, to: 0.6)
                                .stroke(
                                    LinearGradient(
                                        colors: [.blue.opacity(0.5), .purple.opacity(0.5)],
                                        startPoint: .leading,
                                        endPoint: .trailing
                                    ),
                                    style: StrokeStyle(lineWidth: 2, lineCap: .round)
                                )
                                .frame(width: geometry.size.width * 0.6)
                                .rotationEffect(.degrees(rotation))
                            
                            AnimatedNumberText(
                                number: Int(progress * 100),
                                font: .system(size: 36, weight: .bold),
                                textColor: .white
                            )
                            .overlay(
                                Text("%")
                                    .font(.system(size: 20, weight: .bold))
                                    .foregroundColor(.white.opacity(0.8))
                                    .offset(x: 45, y: -8)
                            )
                        }
                    }
                    
                    Spacer()
                    
                    NavigationLink(destination: DoneView(), isActive: $showingDoneView) {
                        EmptyView()
                    }
                }
                .padding(.horizontal)
            }
        }
        .sheet(isPresented: $isShowingFilePicker) {
            DocumentPicker(selectedURL: $selectedURL)
        }
        .onChange(of: selectedURL) { url in
            if url != nil {
                startSync(host: ipAddress)
            }
        }
        .onAppear {
            withAnimation(.linear(duration: 2).repeatForever(autoreverses: false)) {
                rotation = 360
            }
        }
    }
    
    func runSetup(filePath: String, host: String, completion: @escaping (String) -> Void) {
        // Encode the query parameters
        guard let encodedFilePath = filePath.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed),
              let encodedHost = host.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) else {
            completion("Invalid input")
            return
        }
        
        // Construct the URL
        let urlString = "http://localhost:8080/run-setup?file_path=\(encodedFilePath)&host=\(encodedHost)"
        guard let url = URL(string: urlString) else {
            completion("Invalid URL")
            return
        }
        
        // Create the GET request
        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        
        // Perform the request
        URLSession.shared.dataTask(with: request) { data, response, error in
            if let error = error {
                DispatchQueue.main.async {
                    completion("Error: \(error.localizedDescription)")
                }
                return
            }
            
            if let data = data, let responseText = String(data: data, encoding: .utf8) {
                DispatchQueue.main.async {
                    completion("Response: \(responseText)")
                }
            } else {
                DispatchQueue.main.async {
                    completion("No data received")
                }
            }
        }.resume()
    }

    
    private func startSync(host: String) {
        guard let url = selectedURL else { return }
//        appleSync(host: host, port: 1234, localDir: url.path, remoteDir: "files")
        runSetup(filePath: url.path, host: host) {
            responseMessage in
               print(responseMessage)
        }
        simulateProgress()
    }
    
    func simulateProgress() {
        Timer.scheduledTimer(withTimeInterval: 0.05, repeats: true) { timer in
            withAnimation(.easeInOut(duration: 0.2)) {
                if progress < 1.0 {
                    progress += 0.005
                } else {
                    timer.invalidate()
                    DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
                        withAnimation {
                            showingDoneView = true
                        }
                    }
                }
            }
        }
    }
}
