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
                startSync()
            }
        }
        .onAppear {
            withAnimation(.linear(duration: 2).repeatForever(autoreverses: false)) {
                rotation = 360
            }
        }
    }
    
    private func startSync() {
        guard let url = selectedURL else { return }
        appleSync(host: "192.168.1.3", port: 1234, localDir: url.path, remoteDir: "files")
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
struct DocumentPicker: UIViewControllerRepresentable {
    @Binding var selectedURL: URL?
    @Environment(\.presentationMode) private var presentationMode
    
    func makeUIViewController(context: Context) -> UIDocumentPickerViewController {
        // Fixed initialization: asCopy set to false and using proper folder type
        let picker = UIDocumentPickerViewController(forOpeningContentTypes: [.folder], asCopy: false)
        picker.delegate = context.coordinator
        picker.allowsMultipleSelection = false
        picker.shouldShowFileExtensions = true
        return picker
    }
    
    func updateUIViewController(_ uiViewController: UIDocumentPickerViewController, context: Context) {}
    
    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }
    
    class Coordinator: NSObject, UIDocumentPickerDelegate {
        let parent: DocumentPicker
        
        init(_ parent: DocumentPicker) {
            self.parent = parent
        }
        
        func documentPicker(_ controller: UIDocumentPickerViewController, didPickDocumentsAt urls: [URL]) {
            guard let url = urls.first else { return }
            // Get security scoped access to the URL
            let success = url.startAccessingSecurityScopedResource()
            if success {
                parent.selectedURL = url
                // Note: Don't stop accessing the resource here as we need it for sync
                // The app should call url.stopAccessingSecurityScopedResource() when done with the folder
            }
            parent.presentationMode.wrappedValue.dismiss()
        }
        
        func documentPickerWasCancelled(_ controller: UIDocumentPickerViewController) {
            parent.presentationMode.wrappedValue.dismiss()
        }
    }
}

#Preview {
    SyncView()
}
